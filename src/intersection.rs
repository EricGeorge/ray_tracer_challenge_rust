//TODO:  Consider making the Sphere in Interscection a reference
//       to avoid cloning the Sphere in every intersection.
//       This would require a change in the Sphere struct to hold a reference
//       to the Sphere instead of owning it, which may complicate ownership and lifetimes.
//       Alternatively, we could use Arc or Rc to share ownership of the Sphere.

use approx::AbsDiffEq;

use super::point::Point;
use super::ray::Ray;
use super::sphere::Sphere;
use super::vector::Vector;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub s: Sphere,
}

pub struct Computations {
    pub object: Sphere,
    pub point: Point,
    pub eye_vector: Vector,
    pub normal_vector: Vector,
    pub inside: bool,
}

impl Intersection {
    pub fn new(t: f64, s: Sphere) -> Self {
        Self { t, s }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;
        let normal_vector = self.s.normal_at(point);
        let inside = normal_vector.dot(eye_vector) < 0.0;
        let normal_vector = if inside {
            -normal_vector
        } else {
            normal_vector
        };

        Computations {
            object: self.s,
            point,
            eye_vector,
            normal_vector,
            inside,
        }
    }
}

impl AbsDiffEq for Intersection {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-5
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.t.abs_diff_eq(&other.t, epsilon)
    }
}

pub struct Intersections {
    list: Vec<Intersection>,
}

impl Intersections {
    pub fn new(mut intersections: Vec<Intersection>) -> Self {
        intersections
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
        Intersections {
            list: intersections,
        }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.list.iter().find(|&intersection| intersection.t > 0.0)
    }

    pub fn all(&self) -> &[Intersection] {
        &self.list
    }

    pub fn extend(&mut self, other: Intersections) {
        self.list.extend(other.list);
        self.list
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
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
        assert_eq!(i.list[0].s, s);
        assert_eq!(i.list[1].s, s);
    }

    #[test]
    fn hit_all_positive() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);

        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn hit_some_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_lowest_nonnegative() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);

        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        assert_eq!(xs.hit(), Some(&i4));
    }

    #[test]
    fn prepare_computations() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.object, s);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vector, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn prepare_computations_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(r);

        assert!(!comps.inside);
    }

    #[test]
    fn prepare_computations_inside_object() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(1.0, s);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_vector, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }
}
