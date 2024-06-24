mod ascii;

use image::{GenericImageView, DynamicImage};
use std::env;
use std::io::{Write, stdout};
use termion::raw::IntoRawMode;
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
    let img: DynamicImage = image::open(&img_path).expect("Failed to open image");

    // Get the terminal size
    let (term_width, term_height) = terminal_size().unwrap();
    let term_width = term_width as u32;
    let _term_height = term_height as u32;

    // Specify the approximate character height-to-width ratio
    let char_ratio = 2.0; // Height-to-width ratio

    // Calculate the aspect ratio of the image
    let (img_width, img_height) = img.dimensions();
    let aspect_ratio = img_height as f32 / img_width as f32;

    // Adjust aspect ratio for terminal characters
    let adjusted_height = (term_width as f32 * aspect_ratio / char_ratio) as u32;

    // Resize the image to fit terminal width while maintaining aspect ratio
    let resized_img = img.resize_exact(term_width, adjusted_height, image::imageops::FilterType::Nearest);

    // Convert the resized image to grayscale
    let gray_img = resized_img.grayscale();

    let ascii_vec = ascii::get_ascii_from_img_9asciis(&gray_img, true);

    // Print the ASCII art to the terminal
    let mut stdout = stdout().into_raw_mode().unwrap();
    // write!(stdout, "{}", termion::clear::All).unwrap();
    for line in ascii_vec {
        write!(stdout, "{}\r\n", line).unwrap();
    }
    stdout.flush().unwrap();
}