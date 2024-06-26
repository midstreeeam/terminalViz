use image::{GenericImageView, DynamicImage};

#[derive(Debug)]
pub enum AsciiCharset {
    StandardBlock,
    UnicodeBlock,
    NineAscii,
    Custom(Vec<&'static str>),
}

impl AsciiCharset {
    fn get_chars(&self) -> Vec<&'static str> {
        match self {
            AsciiCharset::StandardBlock => vec!["█", "▓", "▒", "░", " "],
            AsciiCharset::UnicodeBlock => vec!["█","▇", "▆", "▅", "▄", "▃", "▂", "▁", " "],
            AsciiCharset::NineAscii => vec!["@", "#", "8", "&", "o", ":", "*", ".", " "],
            AsciiCharset::Custom(chars) => chars.clone(),
        }
    }
}

fn calculate_intensity_range(gray_img: &DynamicImage) -> (u8, u8) {
    let mut min_intensity = 255;
    let mut max_intensity = 0;

    for y in 0..gray_img.height() {
        for x in 0..gray_img.width() {
            let intensity = gray_img.get_pixel(x, y)[0];
            if intensity < min_intensity {
                min_intensity = intensity;
            }
            if intensity > max_intensity {
                max_intensity = intensity;
            }
        }
    }
    (min_intensity, max_intensity)
}

fn generate_ascii_art(gray_img: &DynamicImage, ascii_chars: Vec<&str>, use_all_ascii: bool) -> String {
    let num_chars = ascii_chars.len();
    
    // Calculate the scale
    let ascii_scale = 255 / (num_chars - 1) as u8;

    let (min_intensity, max_intensity) = if use_all_ascii {
        calculate_intensity_range(gray_img)
    } else {
        (0, 255)
    };

    // Create a buffer to hold the ASCII art
    let mut ascii_art = String::new();
    for y in 0..gray_img.height() {
        for x in 0..gray_img.width() {
            let pixel = gray_img.get_pixel(x, y);
            let intensity = pixel[0];
            let scaled_intensity = if use_all_ascii {
                ((intensity - min_intensity) as f64 / (max_intensity - min_intensity) as f64 * (num_chars - 1) as f64) as usize
            } else {
                (intensity / ascii_scale) as usize
            };
            let ascii_char = ascii_chars[scaled_intensity];
            ascii_art.push_str(ascii_char);
        }
        ascii_art.push_str("\r\n");
    }

    ascii_art
}

pub fn get_ascii_from_img(gray_img: &DynamicImage, charset: AsciiCharset, use_all_ascii: bool) -> String {
    let ascii_chars = charset.get_chars();
    generate_ascii_art(gray_img, ascii_chars, use_all_ascii)
}