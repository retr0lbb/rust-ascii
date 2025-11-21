use image::{GenericImageView, Rgba, RgbImage, Rgb};
use imageproc::drawing::draw_text_mut;
use colored::*;
use ab_glyph::{FontArc, PxScale};
use std::{env, error::Error, fs, process::Command, path::PathBuf, thread, time::Duration, io::{Write, BufWriter, stdout}};
use chrono::Utc;
use rayon::prelude::*;

const FONT_RATIO: f32 = 0.5;
const SOBEL_THRESHOLD: f32 = 120.0;

const ASCII_CHARS: &[u8; 16] = b" .:-=+*#%@8BMWS&";

fn build_lut() -> [usize; 256] {
    let mut lut = [0usize; 256];
    for i in 0..256 {
        lut[i] = (i * 15 / 255).min(15);
    }
    lut
}

struct AsciiPixel {
    ch: u8,
    r: u8, g: u8, b: u8,
}

#[inline(always)]
fn luminance(r: u8, g: u8, b: u8) -> u8 {
    ((r as u32 * 54 + g as u32 * 183 + b as u32 * 19) >> 8) as u8
}

#[inline(always)]
fn sobel_at(pixels: &[Rgba<u8>], w: usize, x: usize, y: usize, h: usize) -> (f32, f32) {
    let get = |dx: i32, dy: i32| -> f32 {
        let nx = (x as i32 + dx).clamp(0, w as i32 - 1) as usize;
        let ny = (y as i32 + dy).clamp(0, h as i32 - 1) as usize;
        let px = &pixels[ny * w + nx];
        luminance(px[0], px[1], px[2]) as f32
    };
    
    let gx = -get(-1,-1) + get(1,-1) - 2.0*get(-1,0) + 2.0*get(1,0) - get(-1,1) + get(1,1);
    let gy = -get(-1,-1) - 2.0*get(0,-1) - get(1,-1) + get(-1,1) + 2.0*get(0,1) + get(1,1);
    
    ((gx*gx + gy*gy).sqrt(), gy.atan2(gx))
}

#[inline(always)]
fn angle_to_char(angle: f32) -> u8 {
    let d = angle.to_degrees().rem_euclid(180.0);
    if d < 22.5 || d >= 157.5 { b'-' }
    else if d < 67.5 { b'/' }
    else if d < 112.5 { b'|' }
    else { b'\\' }
}

fn image_to_ascii(img: &image::DynamicImage, cols: u32, sobel: bool, lut: &[usize; 256]) -> Vec<Vec<AsciiPixel>> {
    let (orig_w, orig_h) = img.dimensions();
    let aspect = orig_h as f32 / orig_w as f32;
    let rows = ((cols as f32 * aspect * FONT_RATIO) as u32).max(1);
    
    let resized = img.resize_exact(cols, rows, image::imageops::FilterType::Triangle);
    let rgba = resized.to_rgba8();
    let pixels: Vec<Rgba<u8>> = rgba.pixels().cloned().collect();
    let w = cols as usize;
    let h = rows as usize;

    (0..h).into_par_iter().map(|y| {
        let mut line = Vec::with_capacity(w);
        for x in 0..w {
            let px = &pixels[y * w + x];
            let (r, g, b) = (px[0], px[1], px[2]);
            let lum = luminance(r, g, b);

            let ch = if sobel {
                let (mag, ang) = sobel_at(&pixels, w, x, y, h);
                if mag > SOBEL_THRESHOLD { angle_to_char(ang) } else { ASCII_CHARS[lut[lum as usize]] }
            } else {
                ASCII_CHARS[lut[lum as usize]]
            };

            line.push(AsciiPixel { ch, r, g, b });
        }
        line
    }).collect()
}

fn print_ascii(lines: &[Vec<AsciiPixel>], colored: bool) {
    let mut out = BufWriter::new(stdout().lock());
    for line in lines {
        for px in line {
            if colored {
                write!(out, "{}", (px.ch as char).to_string().truecolor(px.r, px.g, px.b)).ok();
            } else {
                out.write_all(&[px.ch]).ok();
            }
        }
        out.write_all(b"\n").ok();
    }
    out.flush().ok();
}

fn save_ascii_txt(lines: &[Vec<AsciiPixel>], path: &str) -> Result<(), Box<dyn Error>> {
    let cap = lines.len() * (lines.first().map(|l| l.len()).unwrap_or(0) + 1);
    let mut txt = String::with_capacity(cap);
    for line in lines {
        for px in line {
            txt.push(px.ch as char);
        }
        txt.push('\n');
    }
    fs::write(path, txt)?;
    Ok(())
}

fn load_font() -> Result<FontArc, Box<dyn Error>> {
    let font_data = fs::read("font-2.ttf").or_else(|_| fs::read("font.ttf"))?;
    Ok(FontArc::try_from_vec(font_data)?)
}

fn save_ascii_image(lines: &[Vec<AsciiPixel>], path: &str, char_size: u32, font: &FontArc) -> Result<(), Box<dyn Error>> {
    let scale = PxScale::from(char_size as f32);
    
    // Estima dimensões do caractere
    let char_w = (char_size as f32 * 0.6) as u32;
    let char_h = char_size;

    let cols = lines.first().map(|l| l.len()).unwrap_or(0) as u32;
    let rows = lines.len() as u32;
    
    let mut img = RgbImage::new((cols * char_w).max(1), (rows * char_h).max(1));

    for (row, line) in lines.iter().enumerate() {
        for (col, px) in line.iter().enumerate() {
            let x = (col as u32 * char_w) as i32;
            let y = (row as u32 * char_h) as i32;
            let s = (px.ch as char).to_string();
            draw_text_mut(&mut img, Rgb([px.r, px.g, px.b]), x, y, scale, font, &s);
        }
    }

    img.save(path)?;
    Ok(())
}

fn extract_frames(video: &str, out_dir: &str, cols: u32, fps: u32) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(out_dir)?;
    let status = Command::new("ffmpeg")
        .args(["-loglevel", "error", "-i", video, "-vf", &format!("scale={}:-2,fps={}", cols, fps)])
        .arg(format!("{}/frame_%04d.png", out_dir))
        .status()?;
    if !status.success() { return Err("FFmpeg falhou".into()); }
    Ok(())
}

fn process_image(path: &str, cols: u32, colored: bool, save_img: Option<u32>, sobel: bool) -> Result<(), Box<dyn Error>> {
    println!("Processando: {}", path);
    let lut = build_lut();
    
    let img = if path.starts_with("http") {
        let bytes = reqwest::blocking::get(path)?.bytes()?;
        image::load_from_memory(&bytes)?
    } else {
        image::open(path)?
    };

    let lines = image_to_ascii(&img, cols, sobel, &lut);
    print_ascii(&lines, colored);
    
    let stem = PathBuf::from(path).file_stem().and_then(|s| s.to_str()).unwrap_or("output").to_string();
    let ts = Utc::now().timestamp();
    let output_dir = format!("./out/{}_{}", stem, ts);
    fs::create_dir_all(&output_dir)?;
    
    save_ascii_txt(&lines, &format!("{}/ascii.txt", output_dir))?;
    
    if let Some(size) = save_img {
        match load_font() {
            Ok(font) => save_ascii_image(&lines, &format!("{}/ascii.png", output_dir), size, &font)?,
            Err(_) => println!("⚠ Fonte não encontrada, pulando geração de imagem"),
        }
    }
    
    println!("📁 Salvos em: {}", output_dir);
    Ok(())
}

fn process_frames(dir: &str, out_dir: &str, cols: u32, save_img: Option<u32>, sobel: bool) -> Result<(), Box<dyn Error>> {
    let mut frames: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|d| d.path()))
        .filter(|p| p.extension().map(|e| e == "png").unwrap_or(false))
        .collect();
    frames.sort();
    
    let lut = build_lut();
    let total = frames.len();
    
    let font = save_img.and_then(|_| load_font().ok());

    fs::create_dir_all(out_dir)?;
    
    frames.par_iter().enumerate().for_each(|(i, frame)| {
        if i % 10 == 0 {
            print!("\rProcessando: {}/{} ({:.0}%)", i + 1, total, (i + 1) as f32 / total as f32 * 100.0);
            stdout().flush().ok();
        }
        
        if let Ok(img) = image::open(frame) {
            let lines = image_to_ascii(&img, cols, sobel, &lut);
            let stem = frame.file_stem().and_then(|s| s.to_str()).unwrap_or("frame");
            
            save_ascii_txt(&lines, &format!("{}/{}.txt", out_dir, stem)).ok();
            
            if let (Some(size), Some(ref f)) = (save_img, &font) {
                save_ascii_image(&lines, &format!("{}/{}.png", out_dir, stem), size, f).ok();
            }
        }
    });
    
    println!("\n✓ {} frames processados!", total);
    Ok(())
}

fn play_ascii(dir: &str, fps: u32) -> Result<(), Box<dyn Error>> {
    let delay = Duration::from_millis(1000 / fps as u64);
    let mut frames: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|d| d.path()))
        .filter(|p| p.extension().map(|e| e == "txt").unwrap_or(false))
        .collect();
    frames.sort();

    if frames.is_empty() { return Err("Nenhum frame .txt encontrado".into()); }

    let contents: Vec<String> = frames.par_iter()
        .filter_map(|f| fs::read_to_string(f).ok())
        .collect();

    println!("Reproduzindo {} frames (Ctrl+C para sair)...", contents.len());
    loop {
        for content in &contents {
            print!("\x1B[2J\x1B[H{}", content);
            stdout().flush().ok();
            thread::sleep(delay);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(r#"
ASCII Art Converter (Otimizado) - Uso:

  IMAGEM:
    cargo run --release -- imagem.png --image-only [opções]

  VÍDEO:
    cargo run --release -- video.mp4 [opções]

  Opções:
    --image-only      Processa apenas imagem
    --cols N          Largura em caracteres (padrão: 100)
    --color           Saída colorida no terminal
    --sobel           Detecção de bordas
    --save-img N      Salva PNG (N = tamanho fonte)
    --fps N           FPS para vídeo (padrão: 24)
    --play DIR        Reproduz frames de um diretório
"#);
        return Ok(());
    }

    let path = &args[1];
    let image_only = args.contains(&"--image-only".to_string());
    let colored = args.contains(&"--color".to_string());
    let sobel = args.contains(&"--sobel".to_string());

    let get_arg = |flag: &str| -> Option<String> {
        args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
    };

    let cols: u32 = get_arg("--cols").and_then(|s| s.parse().ok()).unwrap_or(100);
    let fps: u32 = get_arg("--fps").and_then(|s| s.parse().ok()).unwrap_or(24);
    let save_img: Option<u32> = get_arg("--save-img").and_then(|s| s.parse().ok());

    if let Some(play_dir) = get_arg("--play") {
        return play_ascii(&play_dir, fps);
    }

    if image_only {
        process_image(path, cols, colored, save_img, sobel)?;
    } else {
        let stem = PathBuf::from(path).file_stem().and_then(|s| s.to_str()).unwrap_or("video").to_string();
        let ts = Utc::now().timestamp();
        let output_dir = format!("./out/{}_{}", stem, ts);
        let frames_dir = format!("{}/frames_raw", output_dir);
        
        println!("Extraindo frames...");
        extract_frames(path, &frames_dir, cols * 10, fps)?;
        
        println!("Convertendo para ASCII...");
        process_frames(&frames_dir, &output_dir, cols, save_img, sobel)?;
        
        println!("📁 Salvos em: {}", output_dir);
        play_ascii(&output_dir, fps)?;
    }

    Ok(())
}