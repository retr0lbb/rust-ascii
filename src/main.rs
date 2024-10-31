use image::GenericImageView;
use colored::*;
use std::env;

fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 16;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@", "#", "$", "%", "&", "8", "B", "M", "W"];
    return ascii[index as usize];
}
 

fn get_image(dir: &str, scale: u32) {
    match image::open(dir) {
        Ok(img) => {
            println!("{:?}", img.dimensions());
            let (width, height) = img.dimensions();

            for y in 0..height{
                for x in 0..width{
                    if y % (scale * 2) == 0 && x % scale == 0{
                        let pix = img.get_pixel(x, y);
                        let mut intent = pix[0]/3 + pix[1]/3 + pix[2]/3;

                        let (r, g, b) = (pix[0], pix[1],pix[2]);


                        if pix[3] == 0{
                            intent = 0;
                        }

                        let ascii_char = get_str_ascii(intent);
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
    let scale = args.get(1).unwrap_or(&"6".to_string()).parse::<u32>().unwrap_or(6);

    get_image("testw.webp", scale);
}
