use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere;

impl Default for Sphere {
    fn default() -> Self {
        Sphere
    }
}

impl Sphere {
    pub fn new() -> Self {
        Sphere
    }

    // Compute the intersection of a ray and a sphere
    // Assumes `ray_obj` is already in object space
    pub fn local_intersect(&self, ray_obj: Ray) -> Option<(f64, f64)> {
        let sphere_to_ray = ray_obj.origin - Point::ORIGIN;
        let a = ray_obj.direction.dot(ray_obj.direction);
        let b = 2.0 * ray_obj.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0_f64; // Sphere::RADIUS.powi(2)

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrt_disc = discriminant.sqrt();
            let q = if b < 0.0 {
                (-b + sqrt_disc) / 2.0
            } else {
                (-b - sqrt_disc) / 2.0
            };
            let t1 = q / a;
            let t2 = c / q;

            // Sort ascending so callers don't care about order.
            let (lo, hi) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
            Some((lo, hi))
        }
    }

    // Object-space normal
    pub fn local_normal_at(&self, p_obj: Point) -> Vector {
        (p_obj - Point::ORIGIN).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::matrix::Transformation;
    use crate::point_light::PointLight;
    use crate::shapes::Shape;
    use crate::vector::Vector;
    use approx::assert_abs_diff_eq;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::new());
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, 4.0);
        assert_abs_diff_eq!(i.all()[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::new());
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, 5.0);
        assert_abs_diff_eq!(i.all()[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::new());
        let i = s.intersect(r);

        assert_eq!(i.all().len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::ORIGIN, Vector::new(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::new());
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, -1.0);
        assert_abs_diff_eq!(i.all()[1].t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::new());
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, -6.0);
        assert_abs_diff_eq!(i.all()[1].t, -4.0);
    }

    #[test]
    fn spheres_default_transformation() {
        let s = Shape::from(Sphere::new());

        assert_eq!(*s.transform(), Matrix::identity());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::from(Sphere::new()).with_transform(Transformation::scaling(2.0, 2.0, 2.0));
        let i = s.intersect(r);

        assert_abs_diff_eq!(i.all()[0].t, 3.0);
        assert_abs_diff_eq!(i.all()[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s =
            Shape::from(Sphere::new()).with_transform(Transformation::translation(5.0, 0.0, 0.0));

        let i = s.intersect(r);

        assert_eq!(i.all().len(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Shape::from(Sphere::new());
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Shape::from(Sphere::new());
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Shape::from(Sphere::new());
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_non_axial_point() {
        let s = Shape::from(Sphere::new());
        let n = s.normal_at(Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));
        let expected = Vector::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(n, expected);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Shape::from(Sphere::new());
        let n = s.normal_at(Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn normal_on_translated_sphere() {
        let s =
            Shape::from(Sphere::new()).with_transform(Transformation::translation(0.0, 1.0, 0.0));

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        let expected = Vector::new(0.0, 0.70711, -0.70711);
        assert_abs_diff_eq!(n, expected);
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let s = Shape::from(Sphere::new()).with_transform(
            Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0),
        );

        let n = s.normal_at(Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0));

        let expected = Vector::new(0.0, 0.97014, 0.24254);
        assert_abs_diff_eq!(n, expected);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Shape::from(Sphere::new());
        assert_eq!(*s.material(), Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let m = Material {
            ambient: 0.1,
            ..Default::default()
        };
        let s = Shape::from(Sphere::new()).with_material(m.clone());
        assert_eq!(*s.material(), m);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let s = Shape::from(Sphere::new());
        let color = s.material().shade(position, light, eye, normal, false);
        assert_eq!(color, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_at_an_angle() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let s = Shape::from(Sphere::new());
        let color = s.material().shade(position, light, eye, normal, false);
        assert_eq!(color, Color::WHITE);
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_and_light() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::WHITE);
        let s = Shape::from(Sphere::new());
        let color = s.material().shade(position, light, eye, normal, false);
        assert_abs_diff_eq!(color, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::WHITE);
        let s = Shape::from(Sphere::new());
        let color = s.material().shade(position, light, eye, normal, false);
        assert_abs_diff_eq!(color, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::WHITE);
        let s = Shape::from(Sphere::new());
        let color = s.material().shade(position, light, eye, normal, false);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let s = Shape::from(Sphere::new());
        let in_shadow = true;
        let color = s.material().shade(position, light, eye, normal, in_shadow);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }
}
