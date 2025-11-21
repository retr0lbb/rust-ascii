use image::{GenericImageView, ImageBuffer, Rgba, RgbImage, Rgb};
use imageproc::drawing::draw_text_mut;
use colored::*;
use rusttype::{Font, Scale};
use std::{env, error::Error, fs, process::Command, path::PathBuf, thread, time::Duration};
use rand::Rng;
use chrono::Utc;

const FONT_RATIO: f32 = 0.5;
const LUMINANCE_THRESHOLD: u8 = 30;

const ASCII_PALETTE: [&str; 16] = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@", "8", "B", "M", "W", "$", "&"];

struct AsciiPixel {
    ch: String,
    color: (u8, u8, u8),
}

fn get_ascii_char(luminance: u8) -> &'static str {
    let idx = (luminance as usize * 15) / 255;
    ASCII_PALETTE[idx.min(15)]
}

fn luminance_from_rgba(px: &Rgba<u8>) -> u8 {
    if px[3] == 0 { return 0; }
    let lum = 0.2126 * px[0] as f32 + 0.7152 * px[1] as f32 + 0.0722 * px[2] as f32;
    lum.clamp(0.0, 255.0) as u8
}

fn sobel_at(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32) -> (f32, f32) {
    let kx: [[i32;3];3] = [[-1,0,1],[-2,0,2],[-1,0,1]];
    let ky: [[i32;3];3] = [[-1,-2,-1],[0,0,0],[1,2,1]];
    let (w, h) = (img.width() as i32, img.height() as i32);
    let (mut gx, mut gy) = (0.0f32, 0.0f32);

    for dy in -1..=1 {
        for dx in -1..=1 {
            let sx = (x + dx).clamp(0, w-1) as u32;
            let sy = (y + dy).clamp(0, h-1) as u32;
            let lum = luminance_from_rgba(img.get_pixel(sx, sy)) as f32;
            gx += kx[(dy+1) as usize][(dx+1) as usize] as f32 * lum;
            gy += ky[(dy+1) as usize][(dx+1) as usize] as f32 * lum;
        }
    }
    ((gx*gx + gy*gy).sqrt(), gy.atan2(gx))
}

fn angle_to_char(angle: f32) -> &'static str {
    let deg = angle.to_degrees();
    let d = ((deg % 360.0) + 360.0) % 360.0;
    match d {
        d if d <= 22.5 || d > 337.5 => "-",
        d if d <= 67.5 => "/",
        d if d <= 112.5 => "|",
        d if d <= 157.5 => "\\",
        d if d <= 202.5 => "-",
        d if d <= 247.5 => "/",
        d if d <= 292.5 => "|",
        _ => "\\",
    }
}

fn image_to_ascii(img: &image::DynamicImage, cols: u32, colored: bool, sobel: bool, print_to_term: bool) -> Vec<Vec<AsciiPixel>> {
    let (orig_w, orig_h) = img.dimensions();
    
    // Calcula dimensões mantendo proporção
    let aspect = orig_h as f32 / orig_w as f32;
    let rows = ((cols as f32 * aspect * FONT_RATIO) as u32).max(1);
    
    let resized = img.resize_exact(cols, rows, image::imageops::FilterType::Lanczos3);
    let rgba = resized.to_rgba8();

    let mut lines: Vec<Vec<AsciiPixel>> = Vec::new();

    for y in 0..rows {
        let mut line: Vec<AsciiPixel> = Vec::new();
        for x in 0..cols {
            let px = rgba.get_pixel(x, y);
            let (r, g, b) = (px[0], px[1], px[2]);
            let lum = luminance_from_rgba(px);

            let ch = if sobel {
                let (mag, ang) = sobel_at(&rgba, x as i32, y as i32);
                if mag > LUMINANCE_THRESHOLD as f32 * 4.0 {
                    angle_to_char(ang).to_string()
                } else {
                    get_ascii_char(lum).to_string()
                }
            } else {
                get_ascii_char(lum).to_string()
            };

            if print_to_term {
                if colored {
                    print!("{}", ch.truecolor(r, g, b));
                } else {
                    print!("{}", ch);
                }
            }

            line.push(AsciiPixel { ch, color: (r, g, b) });
        }
        if print_to_term { println!(); }
        lines.push(line);
    }
    lines
}

fn save_ascii_txt(lines: &[Vec<AsciiPixel>], path: &str) -> Result<(), Box<dyn Error>> {
    let txt: String = lines.iter()
        .map(|line| line.iter().map(|p| p.ch.as_str()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(path, txt)?;
    println!("✓ Salvo TXT: {}", path);
    Ok(())
}

fn save_ascii_image(lines: &[Vec<AsciiPixel>], path: &str, char_size: u32) -> Result<(), Box<dyn Error>> {
    // Usa fonte embutida se font-2.ttf não existir
    let font_data = if let Ok(data) = fs::read("font-2.ttf") {
        data
    } else {
        println!("⚠ font-2.ttf não encontrado, usando fonte padrão");
        (Vec::new())
    };
    
    let font = Font::try_from_vec(font_data).ok_or("Erro ao carregar fonte")?;
    let scale = Scale::uniform(char_size as f32);
    
    let v = font.v_metrics(scale);
    let char_w = font.glyph('W').scaled(scale).h_metrics().advance_width.ceil() as u32;
    let char_h = (v.ascent - v.descent).ceil() as u32;

    let cols = lines.first().map(|l| l.len()).unwrap_or(0) as u32;
    let rows = lines.len() as u32;
    
    let img_w = (cols * char_w).max(1);
    let img_h = (rows * char_h).max(1);

    let mut img = RgbImage::new(img_w, img_h);

    for (row, line) in lines.iter().enumerate() {
        for (col, px) in line.iter().enumerate() {
            let x = (col as u32 * char_w) as i32;
            let y = (row as u32 * char_h) as i32;
            draw_text_mut(&mut img, Rgb([px.color.0, px.color.1, px.color.2]), x, y, scale, &font, &px.ch);
        }
    }

    img.save(path)?;
    println!("✓ Salvo IMG: {}", path);
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
    
    let img = if path.starts_with("http") {
        let bytes = reqwest::blocking::get(path)?.bytes()?;
        image::load_from_memory(&bytes)?
    } else {
        image::open(path)?
    };

    let lines = image_to_ascii(&img, cols, colored, sobel, true);
    
    fs::create_dir_all("./out")?;
    let stem = PathBuf::from(path).file_stem().and_then(|s| s.to_str()).unwrap_or("output").to_string();
    let ts = Utc::now().timestamp();
    
    save_ascii_txt(&lines, &format!("./out/{}_{}.txt", stem, ts))?;
    
    if let Some(size) = save_img {
        save_ascii_image(&lines, &format!("./out/{}_{}.png", stem, ts), size)?;
    }
    Ok(())
}

fn process_frames(dir: &str, cols: u32, colored: bool, save_img: Option<u32>, sobel: bool) -> Result<(), Box<dyn Error>> {
    let mut frames: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|d| d.path()))
        .filter(|p| p.extension().map(|e| e == "png").unwrap_or(false))
        .collect();
    frames.sort();

    fs::create_dir_all("./out")?;

    for (i, frame) in frames.iter().enumerate() {
        print!("\rProcessando frame {}/{}", i + 1, frames.len());
        let img = image::open(frame)?;
        let lines = image_to_ascii(&img, cols, colored, sobel, false);
        
        let stem = frame.file_stem().and_then(|s| s.to_str()).unwrap_or("frame");
        save_ascii_txt(&lines, &format!("./out/{}.txt", stem))?;
        
        if let Some(size) = save_img {
            save_ascii_image(&lines, &format!("./out/{}.png", stem), size)?;
        }
    }
    println!("\n✓ Todos os frames processados!");
    Ok(())
}

fn play_ascii(dir: &str, fps: u32) -> Result<(), Box<dyn Error>> {
    let delay = Duration::from_millis(1000 / fps as u64);
    let mut frames: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|d| d.path()))
        .filter(|p| p.extension().map(|e| e == "txt").unwrap_or(false))
        .collect();
    frames.sort();

    if frames.is_empty() {
        return Err("Nenhum frame .txt encontrado".into());
    }

    println!("Reproduzindo {} frames (Ctrl+C para sair)...", frames.len());
    loop {
        for frame in &frames {
            print!("\x1B[2J\x1B[H{}", fs::read_to_string(frame)?);
            thread::sleep(delay);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(r#"
ASCII Art Converter - Uso:

  IMAGEM:
    cargo run -- imagem.png --image-only [opções]

  VÍDEO:
    cargo run -- video.mp4 [opções]

  Opções:
    --image-only      Processa apenas imagem (não vídeo)
    --cols N          Largura em caracteres (padrão: 100)
    --color           Saída colorida no terminal
    --sobel           Usa detecção de bordas
    --save-img N      Salva como imagem PNG (N = tamanho da fonte)
    --fps N           FPS para vídeo (padrão: 24)
    --play            Apenas reproduz frames existentes em ./out
"#);
        return Ok(());
    }

    let path = &args[1];
    let image_only = args.contains(&"--image-only".to_string());
    let colored = args.contains(&"--color".to_string());
    let sobel = args.contains(&"--sobel".to_string());
    let play_only = args.contains(&"--play".to_string());

    let get_arg = |flag: &str| -> Option<u32> {
        args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)?.parse().ok())
    };

    let cols = get_arg("--cols").unwrap_or(100);
    let fps = get_arg("--fps").unwrap_or(24);
    let save_img = get_arg("--save-img");

    if play_only {
        return play_ascii("./out", fps);
    }

    if image_only {
        process_image(path, cols, colored, save_img, sobel)?;
    } else {
        let frames_dir = format!("./frames_{}", Utc::now().timestamp());
        println!("Extraindo frames para: {}", frames_dir);
        extract_frames(path, &frames_dir, cols * 10, fps)?;
        
        println!("Convertendo para ASCII...");
        process_frames(&frames_dir, cols, colored, save_img, sobel)?;
        
        play_ascii("./out", fps)?;
    }

    Ok(())
}