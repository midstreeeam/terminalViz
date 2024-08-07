#[allow(unused)]
mod ascii;

use gif::DecodeOptions;
use image::{DynamicImage, RgbaImage};
use std::env;
use std::fs::File;
use std::io::{Write, stdout};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::{terminal_size, cursor};

use ascii::AsciiConverter;

fn main() {
    // Get the image path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image_path_or_gif_path>", args[0]);
        return;
    }
    let img_path = &args[1];

    // Ensure the script is executable
    make_script_executable("./change_font_size.sh");

    // Change to the smallest font size, if possible
    change_font_size("6");

    // Check file extension to determine if it's a GIF
    if img_path.ends_with(".gif") {
        display_gif(img_path, 30);
    } else {
        display_image(img_path);
    }

    // Restore original font size, if possible
    change_font_size("12");
}

fn make_script_executable(script_path: &str) {
    let output = Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .output()
        .expect("Failed to make script executable");

    if !output.status.success() {
        eprintln!("Failed to make script executable: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn change_font_size(size: &str) -> bool {
    let script_path = "./change_font_size.sh";  // Adjust the path if necessary

    let output = Command::new("sudo")
        .arg(script_path)
        .arg(size)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to change font size: {}", String::from_utf8_lossy(&output.stderr));
                return false;
            }
        },
        Err(e) => {
            eprintln!("Failed to execute script: {}", e);
            return false;
        }
    }

    true
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

fn display_gif(gif_path: &str, fps: u32) {
    let file = File::open(gif_path).unwrap();

    // Configure the decoder such that it will expand the image to RGBA.
    let mut options = DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = options.read_info(file).unwrap();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let (term_width, _term_height) = terminal_size().unwrap();
    let converter = AsciiConverter::default();

    let mut previous_lines = 0;
    let frame_delay = Duration::from_secs_f32(1.0 / fps as f32);

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        let img = DynamicImage::ImageRgba8(
            RgbaImage::from_raw(frame.width.into(), frame.height.into(), frame.buffer.to_vec()).unwrap()
        );

        let ascii_str = converter.image_to_ascii(img, term_width as u32);
        let lines_count = ascii_str.lines().count();

        // Move cursor up by the number of lines in the previous frame
        if previous_lines > 0 {
            write!(stdout, "{}", cursor::Up((previous_lines + 1) as u16)).unwrap();
        }

        write!(stdout, "{}\r\n", ascii_str).unwrap();
        stdout.flush().unwrap();

        previous_lines = lines_count;

        sleep(frame_delay); // Adjust frame delay based on fps input
    }
}
