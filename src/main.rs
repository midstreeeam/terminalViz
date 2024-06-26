mod ascii;

use image::{GenericImageView, DynamicImage};
use gif::{Decoder, Frame};
use std::env;
use std::fs::File;
use std::io::{Write, stdout, BufReader};
use termion::raw::IntoRawMode;
use termion::terminal_size;
use std::time::Duration;
use std::thread::sleep;

use ascii::AsciiCharset;

fn main() {
    // Get the image path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image_path_or_gif_path>", args[0]);
        return;
    }
    let img_path = &args[1];

    // Check file extension to determine if it's a GIF
    if img_path.ends_with(".gif") {
        display_gif(img_path);
    } else {
        display_image(img_path);
    }
}

fn display_image(img_path: &str) {
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

    let ascii_vec = ascii::get_ascii_from_img(&gray_img, AsciiCharset::UnicodeBlock, true);

    // Print the ASCII art to the terminal
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}\r\n", ascii_vec).unwrap();
    stdout.flush().unwrap();
}

fn display_gif(gif_path: &str) {
    let file = File::open(gif_path).unwrap();
    let mut decoder = Decoder::new(BufReader::new(file)).unwrap();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (term_width, term_height) = terminal_size().unwrap();
    let term_width = term_width as u32;

    let char_ratio = 2.0;

    while let Ok(Some(frame)) = decoder.read_next_frame() {
        let img = DynamicImage::ImageRgba8(image::RgbaImage::from_raw(frame.width.into(), frame.height.into(), frame.buffer.to_vec()).unwrap());

        let (img_width, img_height) = img.dimensions();
        let aspect_ratio = img_height as f32 / img_width as f32;

        let adjusted_height = (term_width as f32 * aspect_ratio / char_ratio) as u32;
        let resized_img = img.resize_exact(term_width, adjusted_height, image::imageops::FilterType::Nearest);
        let gray_img = resized_img.grayscale();

        let ascii_vec = ascii::get_ascii_from_img(&gray_img, AsciiCharset::UnicodeBlock, true);

        write!(stdout, "{}\r\n", ascii_vec).unwrap();
        stdout.flush().unwrap();

        sleep(Duration::from_millis(100)); // Adjust frame delay as needed
    }
}
