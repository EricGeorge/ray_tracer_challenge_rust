use std::ops;

use approx::AbsDiffEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl AbsDiffEq for Color {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-4
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.red.abs_diff_eq(&other.red, epsilon)
            && self.green.abs_diff_eq(&other.green, epsilon)
            && self.blue.abs_diff_eq(&other.blue, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn color_new() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_abs_diff_eq!(c.red, -0.5);
        assert_abs_diff_eq!(c.green, 0.4);
        assert_abs_diff_eq!(c.blue, 1.7);
    }

    #[test]
    fn add_color_to_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn sub_color_from_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_abs_diff_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn mul_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_abs_diff_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
