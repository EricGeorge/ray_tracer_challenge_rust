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

impl Matrix<2> {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix<3> {
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut m = Matrix::ZERO;

        let mut i = 0;

        for r in 0..3 {
            if r == row {
                continue;
            }

            let mut j = 0;
            for c in 0..3 {
                if c == col {
                    continue;
                }

                m[i][j] = self[r][c];
                j += 1;
            }
            i += 1;
        }
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };

        sign * self.minor(row, col)
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for col in 0..3 {
            det += self[0][col] * self.cofactor(0, col);
        }
        det
    }
}

impl Matrix<4> {
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut m = Matrix::ZERO;

        let mut i = 0;

        for r in 0..4 {
            if r == row {
                continue;
            }

            let mut j = 0;
            for c in 0..4 {
                if c == col {
                    continue;
                }

                m[i][j] = self[r][c];
                j += 1;
            }
            i += 1;
        }
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };

        sign * self.minor(row, col)
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for col in 0..4 {
            det += self[0][col] * self.cofactor(0, col);
        }
        det
    }

    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        assert!(!(det == 0.0), "Matrix is not invertible");

        let mut m = Self::ZERO;

        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                m[col][row] = c / det;
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

    #[test]
    fn determinant_2x2() {
        let m = Matrix([[1.0, 5.0], [-3.0, 2.0]]).determinant();
        assert_abs_diff_eq!(m, 17.0);
    }

    #[test]
    fn submatrix_3x3() {
        let m = Matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6., -3.0]]);
        assert_abs_diff_eq!(m.submatrix(0, 2), Matrix([[-3.0, 2.0], [0.0, 6.0]]));
    }

    #[test]
    fn submatrix_4x4() {
        let m = Matrix([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_abs_diff_eq!(
            m.submatrix(2, 1),
            Matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])
        );
    }

    #[test]
    fn minor_3x3() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let s = m.submatrix(1, 0);

        assert_abs_diff_eq!(s.determinant(), 25.0);
        assert_abs_diff_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_3x3() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_abs_diff_eq!(m.minor(0, 0), -12.0);
        assert_abs_diff_eq!(m.cofactor(0, 0), -12.0);

        assert_abs_diff_eq!(m.minor(1, 0), 25.0);
        assert_abs_diff_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3() {
        let m = Matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_abs_diff_eq!(m.cofactor(0, 0), 56.0);
        assert_abs_diff_eq!(m.cofactor(0, 1), 12.0);
        assert_abs_diff_eq!(m.cofactor(0, 2), -46.0);
        assert_abs_diff_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn determinant_4x4() {
        let m = Matrix([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_abs_diff_eq!(m.cofactor(0, 0), 690.0);
        assert_abs_diff_eq!(m.cofactor(0, 1), 447.0);
        assert_abs_diff_eq!(m.cofactor(0, 2), 210.0);
        assert_abs_diff_eq!(m.cofactor(0, 3), 51.0);
        assert_abs_diff_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn check_invertible() {
        let a = Matrix([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_abs_diff_eq!(a.determinant(), -2120.0);
    }

    #[test]
    fn check_noninvertible() {
        let a = Matrix([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_abs_diff_eq!(a.determinant(), 0.0);
    }

    #[test]
    fn inverse_4x4() {
        let a = Matrix([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let inv = a.inverse();

        assert_abs_diff_eq!(a.determinant(), 532.0);
        assert_abs_diff_eq!(a.cofactor(2, 3), -160.0);
        assert_abs_diff_eq!(inv[3][2], -160.0 / 532.0);
        assert_abs_diff_eq!(a.cofactor(3, 2), 105.0);
        assert_abs_diff_eq!(inv[2][3], 105.0 / 532.0);
        assert_abs_diff_eq!(
            inv,
            Matrix([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn inverse_4x4_2() {
        let a = Matrix([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        assert_abs_diff_eq!(
            a.inverse(),
            Matrix([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn multiply_product_inverse() {
        let a = Matrix([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        assert_abs_diff_eq!(a * b * b.inverse(), a);
    }
}
