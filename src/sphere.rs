// use std::ops;

// use approx::AbsDiffEq;

use super::intersection::Intersection;
use super::intersection::Intersections;
use super::point::Point;
use super::ray::Ray;
use super::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let sphere_to_ray: Vector = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b - sqrt_disc) / (2.0 * a);
            let t2 = (-b + sqrt_disc) / (2.0 * a);
            Intersections::new(vec![
                Intersection::new(t1, *self),
                Intersection::new(t2, *self),
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let xs = s.intersect(r);

        assert_eq!(xs.all().len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
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
}
