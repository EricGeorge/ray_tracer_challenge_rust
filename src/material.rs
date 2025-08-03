use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_abs_diff_eq!(m.ambient, 0.1);
        assert_abs_diff_eq!(m.diffuse, 0.9);
        assert_abs_diff_eq!(m.specular, 0.9);
        assert_abs_diff_eq!(m.shininess, 200.0);
    }

    #[test]
    fn custom_material() {
        let m = Material::new(Color::new(0.2, 0.8, 0.7), 0.2, 0.8, 0.7, 150.0);
        assert_abs_diff_eq!(m.ambient, 0.2);
        assert_abs_diff_eq!(m.diffuse, 0.8);
        assert_abs_diff_eq!(m.specular, 0.7);
        assert_abs_diff_eq!(m.shininess, 150.0);
    }
}
