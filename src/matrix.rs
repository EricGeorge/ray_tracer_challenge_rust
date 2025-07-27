use std::ops;

use approx::AbsDiffEq;

use super::point::Point;
use super::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

impl<const N: usize> Matrix<N> {
    const ZERO: Self = Self([[0.0; N]; N]);

    pub fn identity() -> Self {
        let mut m = Self::ZERO;

        for i in 0..N {
            m[i][i] = 1.0;
        }

        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::ZERO;
        for r in 0..N {
            for c in 0..N {
                m[c][r] = self[r][c];
            }
        }
        m
    }
}

impl<const N: usize> ops::Index<usize> for Matrix<N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> ops::Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut m = Self([[0.0; N]; N]);

        for r in 0..N {
            for c in 0..N {
                for i in 0..N {
                    m[r][c] += self[r][i] * other[i][c]
                }
            }
        }
        m
    }
}

impl ops::Mul<Point> for Matrix<4> {
    type Output = Point;

    fn mul(self, other: Point) -> Self::Output {
        // Due to homogeneous coordinates, Point has an implicit 4th value that is
        // equal to 1.0 so it matches the 4x4 Matrix
        Self::Output::new(
            self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z + self[0][3],
            self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z + self[1][3],
            self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z + self[2][3],
        )
    }
}

impl ops::Mul<Vector> for Matrix<4> {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        // Due to homogeneous coordinates, Vector has an implicit 4th value that is
        // equal to 0.0 so it matches the 4x4 Matrix
        Self::Output::new(
            self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z,
            self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z,
            self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z,
        )
    }
}

impl<const N: usize> ops::IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> AbsDiffEq for Matrix<N> {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-4
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(m1, m2)| m1.abs_diff_eq(m2, epsilon))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_abs_diff_ne};

    #[test]
    fn construct_4x4_matrix() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_abs_diff_eq!(m[0][0], 1.0);
        assert_abs_diff_eq!(m[0][3], 4.0);
        assert_abs_diff_eq!(m[1][0], 5.5);
        assert_abs_diff_eq!(m[1][2], 7.5);
        assert_abs_diff_eq!(m[2][2], 11.0);
        assert_abs_diff_eq!(m[3][0], 13.5);
        assert_abs_diff_eq!(m[3][2], 15.5);
    }

    #[test]
    fn construct_2x2_matrix() {
        let m = Matrix([[-3.0, 5.0], [1.0, -2.0]]);

        assert_abs_diff_eq!(m[0][0], -3.0);
        assert_abs_diff_eq!(m[0][1], 5.0);
        assert_abs_diff_eq!(m[1][0], 1.0);
        assert_abs_diff_eq!(m[1][1], -2.0);
    }

    #[test]
    fn construct_3x3_matrix() {
        let m = Matrix([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_abs_diff_eq!(m[0][0], -3.0);
        assert_abs_diff_eq!(m[1][1], -2.0);
        assert_abs_diff_eq!(m[2][2], 1.0);
    }

    #[test]
    fn eq_with_same() {
        let m1 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_abs_diff_eq!(m1, m2);
    }

    #[test]
    fn eq_with_difference() {
        let m1 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_abs_diff_ne!(m1, m2);
    }

    #[test]
    fn multiply_matrices() {
        let m1 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        assert_abs_diff_eq!(
            m1 * m2,
            Matrix([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn multiply_matrix_by_point() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let p = Point::new(1.0, 2.0, 3.0);

        assert_abs_diff_eq!(m * p, Point::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn multiply_matrix_by_vector() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let p = Vector::new(1.0, 2.0, 3.0);

        assert_abs_diff_eq!(m * p, Vector::new(14.0, 22.0, 32.0));
    }

    #[test]
    fn multiply_matriix_by_identity() {
        let m = Matrix([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        let i = Matrix::identity();

        assert_abs_diff_eq!(m * i, m);
    }

    #[test]
    fn multiply_point_by_identity() {
        let p = Point::new(1.0, 2.0, 3.0);
        let i = Matrix::identity();

        assert_abs_diff_eq!(i * p, p);
    }

    #[test]
    fn multiply_vector_by_identity() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let i = Matrix::identity();

        assert_abs_diff_eq!(i * v, v);
    }

    #[test]
    fn transpose_matrix() {
        let m = Matrix([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let t = Matrix([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_abs_diff_eq!(m.transpose(), t)
    }

    #[test]
    fn transpose_identity_matrix() {
        assert_abs_diff_eq!(Matrix::<4>::identity().transpose(), Matrix::<4>::identity());
    }
}
