use std::ops;

use approx::AbsDiffEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    // NEUTRAL COLORS
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

    // PRIMARY COLORS
    pub const RED: Color = Color::new(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);

    // SECONDARY COLORS
    pub const CYAN: Color = Color::new(0.0, 1.0, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0);
    pub const ORANGE: Color = Color::new(1.0, 0.5, 0.0);
    pub const PURPLE: Color = Color::new(0.5, 0.0, 0.5);
    pub const PINK: Color = Color::new(1.0, 0.75, 0.8);

    // EARTH TONES
    pub const BROWN: Color = Color::new(0.59, 0.29, 0.0);
    pub const TAN: Color = Color::new(0.82, 0.71, 0.55);
    pub const OLIVE: Color = Color::new(0.5, 0.5, 0.0);

    // LIGHT/DARK TONES
    pub const DARK_RED: Color = Color::new(0.55, 0.0, 0.0);
    pub const DARK_GREEN: Color = Color::new(0.0, 0.39, 0.0);
    pub const DARK_BLUE: Color = Color::new(0.0, 0.0, 0.55);

    // GRAYS
    pub const LIGHT_GRAY: Color = Color::new(0.83, 0.83, 0.83);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5);
    pub const DARK_GRAY: Color = Color::new(0.25, 0.25, 0.25);

    // ACCENTS
    pub const GOLD: Color = Color::new(1.0, 0.84, 0.0);
    pub const TEAL: Color = Color::new(0.0, 0.5, 0.5);
    pub const CRIMSON: Color = Color::new(0.86, 0.08, 0.24);
    pub const FIREBRICK: Color = Color::new(0.70, 0.13, 0.13);
    pub const TOMATO: Color = Color::new(1.0, 0.39, 0.28);
    pub const GOLDENROD: Color = Color::new(0.85, 0.65, 0.13);
    pub const KHAKI: Color = Color::new(0.94, 0.90, 0.55);
    pub const FOREST_GREEN: Color = Color::new(0.13, 0.55, 0.13);
    pub const SEA_GREEN: Color = Color::new(0.18, 0.55, 0.34);
    pub const LIGHT_SEA_GREEN: Color = Color::new(0.13, 0.70, 0.67);
    pub const TURQUOISE: Color = Color::new(0.25, 0.88, 0.82);
    pub const STEEL_BLUE: Color = Color::new(0.27, 0.51, 0.71);
    pub const ROYAL_BLUE: Color = Color::new(0.25, 0.41, 0.88);
    pub const MIDNIGHT_BLUE: Color = Color::new(0.10, 0.10, 0.44);
    pub const SLATE_GRAY: Color = Color::new(0.44, 0.50, 0.56);
    pub const SADDLE_BROWN: Color = Color::new(0.55, 0.27, 0.07);
    pub const CHOCOLATE: Color = Color::new(0.82, 0.41, 0.12);

    // DEBUG COLORS
    pub const DEBUG_RED: Color = Color::new(1.0, 0.0, 0.0);
    pub const DEBUG_ORANGE: Color = Color::new(1.0, 0.5, 0.0);
    pub const DEBUG_YELLOW: Color = Color::new(1.0, 1.0, 0.0);
    pub const DEBUG_LIME: Color = Color::new(0.5, 1.0, 0.0);
    pub const DEBUG_GREEN: Color = Color::new(0.0, 1.0, 0.0);
    pub const DEBUG_TEAL: Color = Color::new(0.0, 1.0, 0.5);
    pub const DEBUG_CYAN: Color = Color::new(0.0, 1.0, 1.0);
    pub const DEBUG_AZURE: Color = Color::new(0.0, 0.5, 1.0);
    pub const DEBUG_BLUE: Color = Color::new(0.0, 0.0, 1.0);
    pub const DEBUG_VIOLET: Color = Color::new(0.5, 0.0, 1.0);
    pub const DEBUG_MAGENTA: Color = Color::new(1.0, 0.0, 1.0);
    pub const DEBUG_PINK: Color = Color::new(1.0, 0.0, 0.5);

    pub const DEBUG_COLORS: [Color; 12] = [
        Color::DEBUG_RED,
        Color::DEBUG_ORANGE,
        Color::DEBUG_YELLOW,
        Color::DEBUG_LIME,
        Color::DEBUG_GREEN,
        Color::DEBUG_TEAL,
        Color::DEBUG_CYAN,
        Color::DEBUG_AZURE,
        Color::DEBUG_BLUE,
        Color::DEBUG_VIOLET,
        Color::DEBUG_MAGENTA,
        Color::DEBUG_PINK,
    ];

    // Returns a debug color based on the given index.
    // Cycles through the DEBUG_COLORS array if `index` exceeds its length.
    pub fn debug_color_for_index(index: usize) -> Color {
        Self::DEBUG_COLORS[index % Self::DEBUG_COLORS.len()]
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
