// TODO:  Consider moving lighting logic to a separate module
//        to keep the Sphere struct focused on geometry and transformations.
//        This would allow for better separation of concerns and make the code more modular.

use super::color::Color;
use super::intersection::Intersection;
use super::intersection::Intersections;
use super::material::Material;
use super::matrix::Transformation;
use super::point::Point;
use super::point_light::PointLight;
use super::ray::Ray;
use super::vector::Vector;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Transformation,
    material: Material,
    inverse_transform: Transformation,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Transformation::identity(),
            material: Material::default(),
            inverse_transform: Transformation::identity(),
        }
    }
}

impl Sphere {
    const RADIUS: f64 = 1.0;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_transform(mut self, transform: Transformation) -> Self {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    pub fn intersect<'a>(&'a self, ray: Ray) -> Intersections<'a> {
        let transformed_ray = ray.transform(self.inverse_transform);
        let sphere_to_ray = transformed_ray.origin - Point::ORIGIN;
        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - Sphere::RADIUS.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::empty()
        } else {
            let sqrt_disc = discriminant.sqrt();
            let q = if b < 0.0 {
                (-b + sqrt_disc) / 2.0
            } else {
                (-b - sqrt_disc) / 2.0
            };
            let t1 = q / a;
            let t2 = c / q;
            Intersections::new(vec![
                Intersection::new(t1, self),
                Intersection::new(t2, self),
            ])
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let inverse = self.inverse_transform;
        let object_point = inverse * point;
        let object_normal = object_point - Point::ORIGIN;
        let world_normal = inverse.transpose() * object_normal;
        world_normal.normalize()
    }

    pub fn lighting(
        &self,
        position: Point,
        light: PointLight,
        eye: Vector,
        normal: Vector,
    ) -> Color {
        let effective_color = self.material.color * light.intensity;
        let light_vector = (light.position - position).normalize();
        let ambient = effective_color * self.material.ambient;
        let light_dot_normal = light_vector.dot(normal);

        let mut diffuse = Color::BLACK;
        let mut specular = Color::BLACK;

        if light_dot_normal >= 0.0 {
            diffuse = effective_color * self.material.diffuse * light_dot_normal;
            let reflect_vector = -light_vector.reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye);
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.material.shininess);
                specular = light.intensity * self.material.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    use crate::matrix::Transformation;
    use crate::vector::Vector;
    use approx::assert_abs_diff_eq;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, 4.0);
        assert_abs_diff_eq!(i.all()[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, 5.0);
        assert_abs_diff_eq!(i.all()[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = s.intersect(r);

        assert_eq!(i.all().len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::ORIGIN, Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, -1.0);
        assert_abs_diff_eq!(i.all()[1].t, 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = s.intersect(r);
        assert_abs_diff_eq!(i.all()[0].t, -6.0);
        assert_abs_diff_eq!(i.all()[1].t, -4.0);
    }

    #[test]
    fn spheres_default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new().with_transform(Transformation::scaling(2.0, 2.0, 2.0));
        let i = s.intersect(r);

        assert_abs_diff_eq!(i.all()[0].t, 3.0);
        assert_abs_diff_eq!(i.all()[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new().with_transform(Transformation::translation(5.0, 0.0, 0.0));

        let i = s.intersect(r);

        assert_eq!(i.all().len(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_non_axial_point() {
        let s = Sphere::default();
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
        let s = Sphere::default();
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
        let s = Sphere::new().with_transform(Transformation::translation(0.0, 1.0, 0.0));

        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        let expected = Vector::new(0.0, 0.70711, -0.70711);
        assert_abs_diff_eq!(n, expected);
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let s = Sphere::new().with_transform(
            Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0),
        );

        let n = s.normal_at(Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0));

        let expected = Vector::new(0.0, 0.97014, 0.24254);
        assert_abs_diff_eq!(n, expected);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::default();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let m = Material {
            ambient: 0.1,
            ..Default::default()
        };
        let s = Sphere::new().with_material(m);
        assert_eq!(s.material, m);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal);
        assert_eq!(color, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_at_an_angle() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal);
        assert_eq!(color, Color::WHITE);
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_and_light() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal);
        assert_abs_diff_eq!(color, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal);
        assert_abs_diff_eq!(color, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }
}
