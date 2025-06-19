use image::{GenericImageView, RgbImage, Rgb};
use imageproc::drawing::draw_text_mut;
use colored::*;
use rusttype::{Font, Scale, VMetrics};
use core::num;
use std::{env, error::Error, fs};
use rand::Rng;

const ASCII_PALETTE_1: [&str; 16] = [" ", ".", ",", "-", "~", "+", "=", "@", "#", "$", "%", "&", "8", "B", "M", "W"];
const ASCII_PALETTE_2: [&str; 16] = [" ", ":", ";", "!", "^", "*", "x", "o", "O", "0", "Q", "X", "H", "M", "W", "@"];

struct AsciiPixel {
    char: String,
    color: (u8, u8, u8),
}

fn get_str_ascii<'a>(intent: u8, palette: &'a [&'a str; 16]) -> &'a str {
    let index = (intent / 16).min(15);
    palette[index as usize]
}

fn load_image(path_or_url: &str) -> Result<image::DynamicImage, Box<dyn Error>> {
    if path_or_url.starts_with("http") {
        let response = reqwest::blocking::get(path_or_url)?;
        let bytes = response.bytes()?;
        Ok(image::load_from_memory(&bytes)?)
    } else {
        Ok(image::open(path_or_url)?)
    }
}

fn get_image(path_or_url: String, scale: u32, is_colored: bool, output_img: Option<(String, u32)>) {
    match load_image(&path_or_url) {
        Ok(img) => {
            println!("{:?}", img.dimensions());
            let (width, height) = img.dimensions();

            let mut ascii_lines: Vec<Vec<AsciiPixel>> = vec![];

            for y in 0..height {
                if y % (scale * 2) != 0 {
                    continue;
                }

                let mut line: Vec<AsciiPixel> = vec![];

                for x in 0..width {
                    if x % scale != 0 {
                        continue;
                    }

                    let mut rng = rand::thread_rng();
                    let pallet_choice = if rng.gen_bool(0.5) { &ASCII_PALETTE_1 } else { &ASCII_PALETTE_2 };

                    let pix = img.get_pixel(x, y);
                    let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                    let (r, g, b) = (pix[0], pix[1], pix[2]);

                    if pix[3] == 0 {
                        intent = 0;
                    }

                    let ascii_char = get_str_ascii(intent, pallet_choice);

                    if is_colored {
                        print!("{}", ascii_char.truecolor(r, g, b));
                    } else {
                        print!("{}", ascii_char);
                    }

                    line.push(AsciiPixel {
                        char: ascii_char.to_string(),
                        color: (r, g, b),
                    });
                }

                ascii_lines.push(line);
                println!();
            }

            if let Some((ref output_path, char_size)) = output_img {
                save_ascii_to_image(&ascii_lines, output_path, char_size);
                println!("Imagem ASCII salva em: {}", output_path);
            }
        }
        Err(e) => {
            eprintln!("Erro ao abrir imagem: {}", e);
        }
    }
}

fn save_ascii_to_image(ascii_lines: &Vec<Vec<AsciiPixel>>, output_path: &str, char_size: u32) {

    let font_data = fs::read("font-2.ttf").expect("Não foi possível ler a fonte solicitada");
    let font = Font::try_from_vec(font_data).unwrap();
    let scale = Scale { x: char_size as f32, y: char_size as f32 };

    let v_metrics = font.v_metrics(scale);
    let char_width = font.glyph('W').scaled(scale).h_metrics().advance_width;

    let x_spacing = char_width as i32;
    let y_spacing = v_metrics.ascent.abs() as i32 + v_metrics.descent.abs() as i32;

    let img_width = (ascii_lines[0].len() as i32) * x_spacing;
    let img_height = (ascii_lines.len() as i32) * y_spacing;

    let mut img = RgbImage::new(img_width as u32, img_height as u32);


    for (line_idx, line) in ascii_lines.iter().enumerate() {
        let y = (line_idx as i32) * y_spacing;

        for (char_idx, ascii_pixel) in line.iter().enumerate() {
            let x = (char_idx as i32) * x_spacing;
            draw_text_mut(
                &mut img,
                Rgb([ascii_pixel.color.0, ascii_pixel.color.1, ascii_pixel.color.2]),
                x,
                y,
                scale,
                &font,
                &ascii_pixel.char,
            );
        }
    }

    img.save(output_path).expect("Erro ao salvar a imagem de saída");
}

fn generate_random_number(num_of_digits: i32)-> String{
    let mut rng = rand::thread_rng();
    let numero: i32 = rng.gen_range(1..=num_of_digits);  // Gera de 1 a 10, incluindo o 10
    return numero.to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: cargo run <caminho_ou_url_da_imagem> <escala> <color> --img <char_size>");
        return;
    }

    let path_or_url = &args[1];
    let scale = args.get(2).unwrap_or(&"6".to_string()).parse::<u32>().unwrap_or(6);
    let is_colored = args.get(3).map(|arg| arg == "color" || arg == "colored").unwrap_or(false);

    let mut output_img: Option<(String, u32)> = None;

    if let Some(img_flag_pos) = args.iter().position(|arg| arg == "--img") {
        if let Some(size_arg) = args.get(img_flag_pos + 1) {
            if let Ok(char_size) = size_arg.parse::<u32>() {
                let file_name = format!("./out/{}-ascii-{}.png", "image-name", generate_random_number(100));
                output_img = Some((file_name, char_size));
            }
        }
    }

    get_image(path_or_url.to_string(), scale, is_colored, output_img);
}


