use super::vector::Vector;
use approx::AbsDiffEq;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl AbsDiffEq for Point {
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn add_vector_to_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_abs_diff_eq!(Point::new(1.0, 1.0, 6.0), p + v);
    }

    #[test]
    fn sub_point_from_point() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_abs_diff_eq!(Vector::new(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn sub_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_abs_diff_eq!(Point::new(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn mul_scalar() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_abs_diff_eq!(a * 3.5, Point::new(3.5, -7.0, 10.5));
    }

    #[test]
    fn mul_fraction() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_abs_diff_eq!(a * 0.5, Point::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn div_scalar() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_abs_diff_eq!(a / 2.0, Point::new(0.5, -1.0, 1.5));
    }
}
