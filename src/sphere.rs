use super::color::Color;
use super::intersection::Intersection;
use super::intersection::Intersections;
use super::material::Material;
use super::matrix::Transformation;
use super::point::Point;
use super::point_light::PointLight;
use super::ray::Ray;
use super::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
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
            inverse_transform: Transformation::identity(), // cache the inverse
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

    // Compute the intersection of a ray and a sphere
    // note that this function assumes the sphere is at the origin
    // so any transforms need to be made inversely on the ray to
    // transform it from world to object space
    pub fn intersect<'a>(&'a self, ray: Ray) -> Intersections<'a> {
        // *** compute the discriminant ***
        let transformed_ray = ray.transform(self.inverse_transform);
        let sphere_to_ray = transformed_ray.origin - Point::ORIGIN;
        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - Sphere::RADIUS.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        // if the discriminant is negative, there are no intersections
        if discriminant < 0.0 {
            Intersections::empty()
        } else {
            // solve the quadratic for the intersection points
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

    // Compute the normal of the sphere at this point
    // remember that we need to transform the point into object space first
    pub fn normal_at(&self, point: Point) -> Vector {
        let inverse = self.inverse_transform;
        let object_point = inverse * point;
        let object_normal = object_point - Point::ORIGIN;

        // Note:  normal vectors are transformed using the transpose
        // of the inverse of the transformation matrix, because this ensures
        // that the transformed normal vector remains perpendicular to the transformed surface.
        let world_normal = inverse.transpose() * object_normal;
        world_normal.normalize()
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
    pub fn lighting(
        &self,
        position: Point,
        light: PointLight,
        eye: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        // combine the surface color iwth the ligth's color/intensity
        let effective_color = self.material.color * light.intensity;

        // find the direction of the light source
        let light_vector = (light.position - position).normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.material.ambient;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = light_vector.dot(normal);

        let mut diffuse = Color::BLACK;
        let mut specular = Color::BLACK;

        if light_dot_normal >= 0.0 && !in_shadow {
            // compute the diffuse contribution
            diffuse = effective_color * self.material.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflect_vector = -light_vector.reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye);
            if reflect_dot_eye > 0.0 {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.material.shininess);
                specular = light.intensity * self.material.specular * factor;
            }
        }

        // add up all the contributions to get the final shading
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
        let color = s.lighting(position, light, eye, normal, false);
        assert_eq!(color, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_at_an_angle() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal, false);
        assert_eq!(color, Color::WHITE);
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_and_light() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal, false);
        assert_abs_diff_eq!(color, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal, false);
        assert_abs_diff_eq!(color, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::WHITE);
        let s = Sphere::default();
        let color = s.lighting(position, light, eye, normal, false);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let position = Point::ORIGIN;
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::WHITE);
        let s = Sphere::default();
        let in_shadow = true;
        let color = s.lighting(position, light, eye, normal, in_shadow);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }
}
