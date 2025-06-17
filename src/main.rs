use image::{GenericImageView, RgbImage, Rgb};
use imageproc::drawing::draw_text_mut;
use colored::*;
use rusttype::{Font, Scale};
use std::{env, error::Error, fs};
use rand::Rng;

const ASCII_PALETTE_1: [&str; 16] = [" ", ".", ",", "-", "~", "+", "=", "@", "#", "$", "%", "&", "8", "B", "M", "W"];
const ASCII_PALETTE_2: [&str; 16] = [" ", ":", ";", "!", "^", "*", "x", "o", "O", "0", "Q", "X", "H", "M", "W", "@"];


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

fn get_image(path_or_url: &str, scale: u32, is_colored: bool, output_img: Option<&str>) {
    match load_image(path_or_url) {
        Ok(img) => {
            println!("{:?}", img.dimensions());
            let (width, height) = img.dimensions();

            let mut ascii_lines: Vec<String> = vec![];

            for y in 0..height {
                if y % (scale * 2) != 0 {
                    continue;
                }

                let mut line = String::new();

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

                    line.push_str(ascii_char);
                }

                println!();
                ascii_lines.push(line);
            }

            if let Some(output_path) = output_img {
                save_ascii_to_image(&ascii_lines, output_path, 800, 800);
                println!("Imagem ASCII salva em: {}", output_path);
            }
        }
        Err(e) => {
            eprintln!("Erro ao abrir imagem: {}", e);
        }
    }
}

fn save_ascii_to_image(ascii_lines: &Vec<String>, output_path: &str, width: u32, height: u32) {
    let mut img = RgbImage::new(width, height);

    let font_data = fs::read("font.ttf").expect("Não foi possível ler a fonte solicitada");
    let font = Font::try_from_vec(font_data).unwrap();
    let scale = Scale { x: 10.0, y: 10.0 };

    let white = Rgb([255u8, 255u8, 255u8]);

    for (line_idx, line) in ascii_lines.iter().enumerate() {
        let y: i32 = (line_idx as i32) * 12; // 12 pixels de altura por linha
        draw_text_mut(&mut img, white, 5, y, scale, &font, line);
    }

    img.save(output_path).expect("Erro ao salvar a imagem de saída");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<1{
        eprint!("Uso: cargo run <caminho_ou_url_da_imagem> <escala_reduzida> <colorido?>");
        return;
    }
    let path_or_url = &args[1];
    let scale = args.get(2).unwrap_or(&"6".to_string()).parse::<u32>().unwrap_or(6);
    let is_colored = args.get(3).map(|arg| arg == "color" || arg == "colored").unwrap_or(false);


    let output_img = if args.contains(&"--img".to_string()) {
        Some("saida.png")
    } else {
        None
    };
    get_image(path_or_url, scale, is_colored, output_img);
}
