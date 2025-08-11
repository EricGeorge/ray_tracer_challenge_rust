use std::fs;

use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    const PPM_IDENTIFIER: &'static str = "P3";
    const PPM_MAX_COLOR_VALUE: u8 = 255;
    const PPM_MAX_LINE_LEN: u8 = 70;

    pub fn new(width: usize, height: usize, pixels: Vec<Color>) -> Self {
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self::new(width, height, vec![Color::BLACK; width * height])
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x + y * self.width] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[x + y * self.width]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!(
            "{}\n{} {}\n{}\n",
            Self::PPM_IDENTIFIER,
            self.width,
            self.height,
            Self::PPM_MAX_COLOR_VALUE
        );

        for row in self.pixels.chunks(self.width) {
            // Step 1: Flatten row into a Vec<String> of scaled R, G, B values
            let mut color_strings = Vec::new();
            for color in row {
                color_strings.push(Self::scale_to_ppm_data(color.red).to_string());
                color_strings.push(Self::scale_to_ppm_data(color.green).to_string());
                color_strings.push(Self::scale_to_ppm_data(color.blue).to_string());
            }

            // Step 2: Write lines without exceeding PPM_MAX_LINE_LEN
            let mut line = String::new();
            let mut line_len = 0;

            for color in color_strings.into_iter() {
                let color_len = color.len();
                let sep = if line_len == 0 { "" } else { " " };
                let addition_len = sep.len() + color_len;

                if line_len + addition_len > Self::PPM_MAX_LINE_LEN as usize {
                    ppm.push_str(&line);
                    ppm.push('\n');
                    line.clear();
                    line_len = 0;
                }

                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(&color);
                line_len += addition_len;
            }

            // Push any remaining line content
            if !line.is_empty() {
                ppm.push_str(&line);
                ppm.push('\n');
            }
        }
        ppm
    }

    pub fn scale_to_ppm_data(color_scale: f64) -> u8 {
        let max_color_val = f64::from(Self::PPM_MAX_COLOR_VALUE);
        let scaled_data = color_scale * max_color_val;

        scaled_data.clamp(0.0, max_color_val).round() as u8
    }

    pub fn write_ppm(&self, filename: &str) {
        let ppm = self.to_ppm();

        fs::create_dir_all(std::path::Path::new(filename).parent().unwrap())
            .expect("directory should be created successfully");
        fs::write(filename, ppm).expect("file should be written successfully");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn new() {
        let canvas = Canvas::empty(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        for pixel in canvas.pixels {
            assert_abs_diff_eq!(pixel.red, 0.0);
            assert_abs_diff_eq!(pixel.blue, 0.0);
            assert_abs_diff_eq!(pixel.green, 0.0);
        }
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::empty(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, red);

        assert_eq!(canvas.pixel_at(2, 3), red);
    }

    #[test]
    fn ppm_header() {
        let canvas = Canvas::empty(5, 3);

        let ppm = canvas.to_ppm();

        let mut lines = ppm.lines();
        assert_eq!(lines.next().unwrap(), "P3");
        assert_eq!(lines.next().unwrap(), "5 3");
        assert_eq!(lines.next().unwrap(), "255");
    }

    #[test]
    fn ppm_pixel_data() {
        let mut canvas = Canvas::empty(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let ppm = canvas.to_ppm();

        let mut lines = ppm.lines();
        lines.next();
        lines.next();
        lines.next();

        assert_eq!(lines.next().unwrap(), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines.next().unwrap(), "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines.next().unwrap(), "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        assert!(lines.next().is_none());
    }

    #[test]
    fn ppm_split_long_lines() {
        let mut canvas = Canvas::empty(10, 2);

        for row in 0..canvas.height {
            for col in 0..canvas.width {
                canvas.write_pixel(col, row, Color::new(1.0, 0.8, 0.6));
            }
        }

        let ppm = canvas.to_ppm();

        let mut lines = ppm.lines();
        lines.next();
        lines.next();
        lines.next();

        assert_eq!(
            lines.next().unwrap(),
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines.next().unwrap(),
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines.next().unwrap(),
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines.next().unwrap(),
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ppm_terminate_with_newline() {
        let canvas = Canvas::empty(5, 3);

        let ppm = canvas.to_ppm();

        assert!(ppm.ends_with('\n'));
    }
}
