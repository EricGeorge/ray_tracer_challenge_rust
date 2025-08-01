// use std::ops;

// use approx::AbsDiffEq;

use super::point::Point;
use super::ray::Ray;
use super::vector::Vector;

#[derive(Debug)]
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

    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
        let str: Vector = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&str);
        let c = str.dot(&str) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
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
        let xs = s.intersect(r);
        assert_abs_diff_eq!(xs[0], 4.0);
        assert_abs_diff_eq!(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_abs_diff_eq!(xs[0], 5.0);
        assert_abs_diff_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_abs_diff_eq!(xs[0], -1.0);
        assert_abs_diff_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_abs_diff_eq!(xs[0], -6.0);
        assert_abs_diff_eq!(xs[1], -4.0);
    }
}
