extern crate image;
extern crate termion;

use image::GenericImageView;
use std::env;
// use std::fs::File;
// use std::io::BufReader;
use termion::terminal_size;

fn main() {
    // Get the image path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return;
    }
    let img_path = &args[1];

    // Open the image file
    let img = image::open(&img_path).expect("Failed to open image");

    // Get terminal size
    let (term_width, term_height) = terminal_size().expect("Failed to get terminal size");
    let term_width = term_width as u32;
    let _term_height = term_height as u32 * 2; // Adjusting for character aspect ratio

    // Calculate the new image dimensions
    let (img_width, img_height) = img.dimensions();
    let aspect_ratio = img_width as f32 / img_height as f32;
    let new_width = term_width;
    let new_height = (new_width as f32 / aspect_ratio) as u32;

    // Resize the image
    let resized_img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

    // Convert the image to grayscale
    let grayscale_img = resized_img.to_luma8();

    // ASCII characters used for mapping
    let ascii_chars = ["@", "#", "8", "&", "o", ":", "*", ".", " "];

    // Generate ASCII art
    let mut ascii_art = String::new();
    for y in 0..new_height {
        for x in 0..new_width {
            let pixel = grayscale_img.get_pixel(x, y).0[0];
            let ascii_index = (pixel as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
            ascii_art.push_str(ascii_chars[ascii_index]);
        }
        ascii_art.push('\n');
    }

    // Print ASCII art to the terminal
    print!("{}", ascii_art);
}
