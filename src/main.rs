use image::GenericImageView;
use colored::*;
use std::{env, error::Error};
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

fn get_image(path_or_url: &str, scale: u32) {
    match load_image(path_or_url) {
        Ok(img) => {
            println!("{:?}", img.dimensions());
            let (width, height) = img.dimensions();

            for y in 0..height{
                for x in 0..width{
                    let mut rng = rand::thread_rng();
                    let pallet_choice = if rng.gen_bool(0.5) {&ASCII_PALETTE_1} else {&ASCII_PALETTE_2};
                    if y % (scale * 2) == 0 && x % scale == 0{
                        let pix = img.get_pixel(x, y);
                        let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;

                        let (r, g, b) = (pix[0], pix[1],pix[2]);


                        if pix[3] == 0{
                            intent = 0;
                        }

                        let ascii_char = get_str_ascii(intent, pallet_choice);
                        print!("{}", ascii_char.truecolor(r, g, b))
                    }
                }
            
                if y %(scale*2) == 0{
                    println!("")
                }
            }
        }
        Err(e) =>{
            eprintln!("Erro ao abrir imagem: {}", e)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<3{
        eprint!("Uso: cargo run <caminho_ou_url_da_imagem> <escala_reduzida> <colorido?>");
        return;
    }
    let path_or_url = &args[1];
    let scale = args.get(2).unwrap_or(&"6".to_string()).parse::<u32>().unwrap_or(6);
    let is_colored = args.get(3).unwrap();


    get_image(path_or_url, scale);
}
