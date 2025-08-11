use crate::matrix::Transformation;
use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
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

    pub fn transform(&self, m: Transformation) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
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

    #[test]
    fn translating_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let transformed_ray = r.transform(m);

        assert_abs_diff_eq!(transformed_ray.origin, Point::new(4.0, 6.0, 8.0));
        assert_abs_diff_eq!(transformed_ray.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let transformed_ray = r.transform(m);

        assert_abs_diff_eq!(transformed_ray.origin, Point::new(2.0, 6.0, 12.0));
        assert_abs_diff_eq!(transformed_ray.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
