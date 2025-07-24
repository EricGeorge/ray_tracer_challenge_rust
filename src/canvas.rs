use super::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, pixels: Vec<Color>) -> Self {
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self::new(
            width,
            height,
            vec![Color::new(0.0, 0.0, 0.0); width * height],
        )
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
}
