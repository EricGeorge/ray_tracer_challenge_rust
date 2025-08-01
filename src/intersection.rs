// use std::ops;

// use approx::AbsDiffEq;

// use super::point::Point;
// use super::vector::Vector;
use super::sphere::Sphere;

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub s: Sphere,
}

impl Intersection {
    pub fn new(t: f64, s: Sphere) -> Self {
        Self { t, s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::ray::Ray;
    use crate::vector::Vector;
    use approx::assert_abs_diff_eq;

    #[test]
    fn create_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, s);

        assert_abs_diff_eq!(i.t, 3.5);
        assert_eq!(i.s, s);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = s.intersect(r);
        assert_eq!(i[0].s, s);
        assert_eq!(i[1].s, s);
    }
}
