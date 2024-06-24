use image::{GenericImageView, DynamicImage};

pub fn get_ascii_from_img_4squares(gray_img: &DynamicImage, use_all_ascii: bool) -> Vec<String> {
    // Map grayscale values to ASCII characters
    let ascii_chars = vec!["█", "▓", "▒", "░", " "];
    let num_chars = ascii_chars.len();
    
    // Calculate the scale
    let ascii_scale = 255 / (num_chars - 1) as u8;

    let (min_intensity, max_intensity) = if use_all_ascii {
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
    } else {
        (0, 255)
    };

    // Create a buffer to hold the ASCII art
    let mut ascii_art = Vec::new();
    for y in 0..gray_img.height() {
        let mut line = String::new();
        for x in 0..gray_img.width() {
            let pixel = gray_img.get_pixel(x, y);
            let intensity = pixel[0];
            let scaled_intensity = if use_all_ascii {
                ((intensity - min_intensity) as f64 / (max_intensity - min_intensity) as f64 * (num_chars - 1) as f64) as usize
            } else {
                (intensity / ascii_scale) as usize
            };
            let ascii_char = ascii_chars[scaled_intensity];
            line.push_str(ascii_char);
        }
        ascii_art.push(line);
    }

    ascii_art
}

pub fn get_ascii_from_img_9asciis(gray_img: &DynamicImage, use_all_ascii: bool) -> Vec<String> {
    // Map grayscale values to ASCII characters
    let ascii_chars = vec!["@", "#", "8", "&", "o", ":", "*", ".", " "];
    let num_chars = ascii_chars.len();
    
    // Calculate the scale
    let ascii_scale = 255 / (num_chars - 1) as u8;

    let (min_intensity, max_intensity) = if use_all_ascii {
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
    } else {
        (0, 255)
    };

    // Create a buffer to hold the ASCII art
    let mut ascii_art = Vec::new();
    for y in 0..gray_img.height() {
        let mut line = String::new();
        for x in 0..gray_img.width() {
            let pixel = gray_img.get_pixel(x, y);
            let intensity = pixel[0];
            let scaled_intensity = if use_all_ascii {
                ((intensity - min_intensity) as f64 / (max_intensity - min_intensity) as f64 * (num_chars - 1) as f64) as usize
            } else {
                (intensity / ascii_scale) as usize
            };
            let ascii_char = ascii_chars[scaled_intensity];
            line.push_str(ascii_char);
        }
        ascii_art.push(line);
    }

    ascii_art
}
