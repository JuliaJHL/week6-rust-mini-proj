use std::{env};
use image::{GenericImageView, ImageBuffer, ImageFormat, io::Reader as ImageReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Welcome to Image Resizing!");
        println!("Usage: cargo run <input_path> <output_path> <new_width>");
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let new_width = args[3].parse::<u32>().expect("Invalid width");

    // get the input image from input_path
    let img = match ImageReader::open(input_path) {
        Ok(img) => img,
        Err(e) => {
            println!("Error opening image: {}", e);
            return;
        }
    }.decode().expect("Error decoding image");
    // resize the image
    let (width, height) = img.dimensions();
    let new_height = (height as f64 * new_width as f64 / width as f64) as u32;
    let resized_img = ImageBuffer::from_fn(new_width, new_height, |x, y| {
        let x = (x as f64 * width as f64 / new_width as f64).floor() as u32;
        let y = (y as f64 * height as f64 / new_height as f64).floor() as u32;
        img.get_pixel(x, y)
    });

    let _result = match resized_img.save_with_format(&output_path, ImageFormat::Png){
        Ok(_result) => println!("Image resized and saved successfully"),
        Err(e) => {
            println!("Error saving image: {}", e);
            return;
        }
    };
}
