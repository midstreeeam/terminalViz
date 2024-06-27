#[allow(unused)]
mod ascii;

use image::DynamicImage;
use std::env;
use std::fs::File;
use std::io::{Write, stdout};
use termion::raw::IntoRawMode;
use termion::{terminal_size, cursor};
use std::time::Duration;
use std::thread::sleep;

use ascii::AsciiConverter;

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
    let converter = AsciiConverter::default();

    let (term_width, _term_height) = terminal_size().unwrap();
    let ascii_str = converter.image_to_ascii(img, term_width as u32);

    // Print the ASCII art to the terminal
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}\r\n", ascii_str).unwrap();
    stdout.flush().unwrap();
}

fn display_gif(gif_path: &str) {
    let file = File::open(gif_path).unwrap();

    // Configure the decoder such that it will expand the image to RGBA.
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = options.read_info(file).unwrap();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let (term_width, _term_height) = terminal_size().unwrap();
    let converter = AsciiConverter::default();

    let mut previous_lines = 0;

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        let img = DynamicImage::ImageRgba8(
            image::RgbaImage::from_raw(frame.width.into(), frame.height.into(), frame.buffer.to_vec()).unwrap()
        );

        let ascii_str = converter.image_to_ascii(img, term_width as u32);
        let lines_count = ascii_str.lines().count();

        // Move cursor up by the number of lines in the previous frame
        if previous_lines > 0 {
            write!(stdout, "{}", cursor::Up((previous_lines+1) as u16)).unwrap();
        }

        write!(stdout, "{}\r\n", ascii_str).unwrap();
        stdout.flush().unwrap();

        previous_lines = lines_count;

        sleep(Duration::from_millis(100)); // Adjust frame delay as needed
    }
}
