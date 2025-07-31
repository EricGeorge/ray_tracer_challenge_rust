// use std::ops;

// use approx::AbsDiffEq;

use super::point::Point;
use super::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn create_ray() {
        let o = Point::new(1.0, 2.0, 3.0);
        let d = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(o, d);
        assert_abs_diff_eq!(r.origin, o);
        assert_abs_diff_eq!(r.direction, d);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_abs_diff_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_abs_diff_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_abs_diff_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_abs_diff_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }
}
