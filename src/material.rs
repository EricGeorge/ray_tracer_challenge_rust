use crate::color::Color;
use crate::pattern::Pattern;
use crate::point::Point;
use crate::point_light::PointLight;
use crate::shapes::Shape;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<Pattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn new(
        color: Color,
        pattern: Option<Pattern>,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Self {
        Self {
            color,
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    // calculate the lighting at the position on the sphere using the Phong Reflection Model
    //
    // Ambient reflection is background lighting, or light reflected from other
    // objects in the environment. The Phong model treats this as a constant,
    // coloring all points on the surface equally.
    //
    // Diffuse reflection is light reflected from a matte surface. It depends only
    // on the angle between the light source and the surface normal.
    //
    // Specular reflection is the reflection of the light source itself and results in
    // what is called a specular highlight—the bright spot on a curved surface.
    // It depends only on the angle between the reflection vector and the eye
    // vector and is controlled by a parameter that we’ll call shininess. The
    // higher the shininess, the smaller and tighter the specular highlight.
    pub fn shade(
        &self,
        object: &Shape,
        position: Point,
        light: PointLight,
        eye: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        // combine the surface color iwth the ligth's color/intensity
        let effective_color = if let Some(pattern) = &self.pattern {
            pattern.pattern_at_object(object, position)
        } else {
            self.color * light.intensity
        };

        // find the direction of the light source
        let light_vector = (light.position - position).normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = light_vector.dot(normal);

        let mut diffuse = Color::BLACK;
        let mut specular = Color::BLACK;

        if light_dot_normal >= 0.0 && !in_shadow {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_vector = -light_vector.reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye);
            if reflect_dot_eye > 0.0 {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        // add up all the contributions to get the final shading
        ambient + diffuse + specular
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
        let m = Material::new(Color::new(0.2, 0.8, 0.7), None, 0.2, 0.8, 0.7, 150.0);
        assert_abs_diff_eq!(m.ambient, 0.2);
        assert_abs_diff_eq!(m.diffuse, 0.8);
        assert_abs_diff_eq!(m.specular, 0.7);
        assert_abs_diff_eq!(m.shininess, 150.0);
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let pattern = Pattern::striped(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0));
        let m = Material::new(Color::new(1.0, 1.0, 1.0), Some(pattern), 1.0, 0.0, 0.0, 0.0);
        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let c1 = m.shade(
            &Shape::sphere(),
            Point::new(0.9, 0.0, 0.0),
            light,
            eye_vector,
            normal_vector,
            false,
        );
        let c2 = m.shade(
            &Shape::sphere(),
            Point::new(1.1, 0.0, 0.0),
            light,
            eye_vector,
            normal_vector,
            false,
        );

        assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
    }
}
