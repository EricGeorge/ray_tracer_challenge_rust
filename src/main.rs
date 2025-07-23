// TODO - re-enable dead_code warning
// TODO - Refactor Tuple into Point and Vector and fix up test
// TODO - Convert to a crate with both a library and multiple binaries
#![allow(dead_code)]

use approx::AbsDiffEq;
use std::ops;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

fn is_point(t: &Tuple) -> bool {
    t.w == 1.0
}

fn is_vector(t: &Tuple) -> bool {
    t.w == 0.0
}

fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        if is_point(&self) && is_point(&other) {
            panic!("Adding two points is not allowed!");
        }
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}
impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.abs_diff_eq(other, Self::default_epsilon())
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-4
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
    }
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let m = self.magnitude();
        tuple(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: &Self) -> Self {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn tuple_as_point() {
        let t = Tuple {
            x: 4.3,
            y: 4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(is_point(&t));
        assert!(!is_vector(&t));
    }

    #[test]
    fn tuple_as_vector() {
        let t = Tuple {
            x: 4.3,
            y: 4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(!is_point(&t));
        assert!(is_vector(&t));
    }

    #[test]
    fn create_point() {
        assert_abs_diff_eq!(
            point(4.0, -4.0, 3.0),
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn create_vector() {
        assert_abs_diff_eq!(
            vector(4.0, -4.0, 3.0),
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            }
        );
    }

    #[test]
    fn add_vector_to_point() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);
        assert_abs_diff_eq!(tuple(1.0, 1.0, 6.0, 1.0), p + v);
    }

    #[test]
    fn subtract_point_from_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_abs_diff_eq!(vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_abs_diff_eq!(point(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtract_vector_from_vector() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_abs_diff_eq!(vector(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn neg_tuple() {
        let t = tuple(1.0, -2.0, 3.0, -4.0);
        let neg_t = tuple(-1.0, 2.0, -3.0, 4.0);
        assert_abs_diff_eq!(-t, neg_t);
    }

    #[test]
    fn mult_tuple_by_scaler() {
        let t1 = tuple(1.0, -2.0, 3.0, -4.0);
        let t2 = tuple(3.5, -7.0, 10.5, -14.0);
        let t3 = tuple(0.5, -1.0, 1.5, -2.0);

        assert_abs_diff_eq!(t1 * 3.5, t2);
        assert_abs_diff_eq!(t1 * 0.5, t3);
    }

    #[test]
    fn div_tuple_by_scaler() {
        let t1 = tuple(1.0, -2.0, 3.0, -4.0);
        let t2 = tuple(0.5, -1.0, 1.5, -2.0);

        assert_abs_diff_eq!(t1 / 2.0, t2);
    }

    #[test]
    fn vector_magnitude() {
        let v1 = vector(1.0, 0.0, 0.0);
        assert_abs_diff_eq!(v1.magnitude(), 1.0);

        let v2 = vector(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(v2.magnitude(), 14.0_f64.sqrt());

        let v3 = vector(-1.0, -2.0, -3.0);
        assert_abs_diff_eq!(v3.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn vector_normalize() {
        let v1 = vector(4.0, 0.0, 0.0);
        assert_abs_diff_eq!(v1.normalize(), vector(1.0, 0.0, 0.0));

        let v2 = vector(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(
            v2.normalize(),
            vector(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        )
    }

    #[test]
    fn vector_normalize_magnitude() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_abs_diff_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_abs_diff_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_abs_diff_eq!(v1.cross(&v2), vector(-1.0, 2.0, -1.0));
        assert_abs_diff_eq!(v2.cross(&v1), vector(1.0, -2.0, 1.0));
    }
}
