use crate::intersection::LocalHits;
use crate::point::Point;
use crate::ray::Ray;
use crate::utils::EPSILON;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Plane;

impl Default for Plane {
    fn default() -> Self {
        Plane
    }
}

impl Plane {
    pub fn new() -> Self {
        Plane
    }

    // Compute the intersection of a ray and a Plane
    // Assumes `ray` is already in object space
    pub fn local_intersect(&self, ray: Ray) -> LocalHits {
        if ray.direction.y.abs() < EPSILON {
            LocalHits::None // parallel
        } else {
            let t = -ray.origin.y / ray.direction.y;
            LocalHits::One(t)
        }
    }

    // Object-space normal
    pub fn local_normal_at(&self, _point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0) // Normal for a plane is always (0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Point::new(-5.0, 0.0, 150.0));

        assert_abs_diff_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersecting_parallel_ray_with_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(p.local_intersect(r), LocalHits::None);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(p.local_intersect(r), LocalHits::None);
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        assert_eq!(p.local_intersect(r), LocalHits::One(1.0));
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(p.local_intersect(r), LocalHits::One(1.0));
    }
}
