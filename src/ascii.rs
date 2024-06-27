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

pub struct AsciiConverter {
    char_ratio: f32,
    char_set: AsciiCharset,
    use_all_ascii: bool
}

impl Default for AsciiConverter {
    fn default() -> Self {
        AsciiConverter { 
            char_ratio: 2.0, 
            char_set: AsciiCharset::UnicodeBlock, 
            use_all_ascii: true 
        }
    }
}

impl AsciiConverter {

    pub fn image_to_ascii(&self, img: DynamicImage, term_width: u32) -> String {
        // Calculate the aspect ratio of the image
        let (img_width, img_height) = img.dimensions();
        let aspect_ratio = img_height as f32 / img_width as f32;

        // Adjust aspect ratio for terminal characters
        let adjusted_height = (term_width as f32 * aspect_ratio / self.char_ratio) as u32;

        // Resize the image to fit terminal width while maintaining aspect ratio
        let resized_img = img.resize_exact(term_width, adjusted_height, image::imageops::FilterType::Nearest);

        // Convert the resized image to grayscale
        let gray_img = resized_img.grayscale();

        // Convert the grayscale image to ASCII
        get_ascii(&gray_img, &self.char_set, self.use_all_ascii)
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

fn get_ascii(gray_img: &DynamicImage, charset: &AsciiCharset, use_all_ascii: bool) -> String {
    let ascii_chars = charset.get_chars();
    generate_ascii_art(gray_img, ascii_chars, use_all_ascii)
}