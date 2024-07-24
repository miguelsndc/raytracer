use crate::primitives::{float::ApproxEq, matrix3f::Matrix3f, tuple::Tuple};

const MATRIX_SIZE: usize = 4;

#[derive(Clone, Copy, Debug)]
pub struct Matrix4f {
    inner: [f64; MATRIX_SIZE * MATRIX_SIZE],
}

impl Matrix4f {
    pub fn new() -> Self {
        Self {
            inner: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }

    pub fn identity() -> Self {
        Self {
            inner: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
    pub fn new_from(layout: [[f64; MATRIX_SIZE]; MATRIX_SIZE]) -> Self {
        let mut m = Self::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                m[(i, j)] = layout[i][j];
            }
        }
        return m;
    }

    pub fn transpose(&self) -> Self {
        let mut m = Matrix4f::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                m[(j, i)] = self[(i, j)];
            }
        }
        return m;
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3f {
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);

        let mut m = Matrix3f::new();
        let mut u = 0;

        for i in 0..MATRIX_SIZE {
            if i != row {
                let mut v = 0;
                for j in 0..MATRIX_SIZE {
                    if j != col {
                        m[(u, v)] = self[(i, j)];
                        v += 1;
                    }
                }
                u += 1;
            }
        }

        return m;
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        let m = self.submatrix(row, col);
        return m.determinant();
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 0 {
            return m;
        } else {
            return -m;
        }
    }

    pub fn determinant(&self) -> f64 {
        let det = self.cofactor(0, 0) * self[(0, 0)]
            + self.cofactor(0, 1) * self[(0, 1)]
            + self.cofactor(0, 2) * self[(0, 2)]
            + self.cofactor(0, 3) * self[(0, 3)];
        return det;
    }

    pub fn invert(&self) -> Option<Matrix4f> {
        let det = self.determinant();

        if det == 0.0 {
            return None;
        }

        let mut inverse = Matrix4f::new();

        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                inverse[(j, i)] = self.cofactor(i, j) / det;
            }
        }

        return Some(inverse);
    }
}
impl PartialEq for Matrix4f {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                if !self.inner[i * MATRIX_SIZE + j]
                    .approx_eq_low_precision(other.inner[i * MATRIX_SIZE + j])
                {
                    return false;
                }
            }
        }
        return true;
    }
}

impl std::ops::Index<(usize, usize)> for Matrix4f {
    type Output = f64;
    fn index(&self, indexes: (usize, usize)) -> &f64 {
        let (row, col) = indexes;
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        return &self.inner[row * MATRIX_SIZE + col];
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix4f {
    fn index_mut(&mut self, indexes: (usize, usize)) -> &mut f64 {
        let (row, col) = indexes;
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        return &mut self.inner[row * MATRIX_SIZE + col];
    }
}

impl std::ops::Mul for Matrix4f {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut m = Matrix4f::new();
        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                m[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }
        return m;
    }
}

impl<T> std::ops::Mul<T> for Matrix4f
where
    T: Tuple,
{
    type Output = T;
    fn mul(self, rhs: T) -> T {
        let x = self[(0, 0)] * rhs.x()
            + self[(0, 1)] * rhs.y()
            + self[(0, 2)] * rhs.z()
            + self[(0, 3)] * rhs.w();
        let y = self[(1, 0)] * rhs.x()
            + self[(1, 1)] * rhs.y()
            + self[(1, 2)] * rhs.z()
            + self[(1, 3)] * rhs.w();
        let z = self[(2, 0)] * rhs.x()
            + self[(2, 1)] * rhs.y()
            + self[(2, 2)] * rhs.z()
            + self[(2, 3)] * rhs.w();

        return T::new(x, y, z);
    }
}

#[cfg(test)]
mod tests {
    use super::super::vec3::Vec3;
    use super::*;

    #[test]
    fn matrix4f_initialization() {
        let m = Matrix4f::new_from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [12.1, 13.0, 4.0, 1.0],
            [-1.0, 8.0, 9.0, 10.0],
        ]);

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 1)], 6.5);
        assert_eq!(m[(2, 2)], 4.0);
        assert_eq!(m[(3, 3)], 10.0);
    }

    #[test]
    fn matrix4f_identity() {
        let m = Matrix4f::identity();

        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                if i == j {
                    assert_eq!(m[(i, j)], 1.0);
                } else {
                    assert_eq!(m[(i, j)], 0.0);
                }
            }
        }
    }

    #[test]
    fn matrix4f_equality() {
        let m1 = Matrix4f::new_from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [12.1, 13.0, 4.0, 1.0],
            [-1.0, 8.0, 9.0, 10.0],
        ]);

        let m3 = Matrix4f::new_from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [12.1, 13.0, 4.0, 1.0],
            [-1.0, 8.0, 9.0, 10.0],
        ]);

        let m2 = Matrix4f::identity();

        assert!(m1 != m2);
        assert!(m1 == m3);
    }

    #[test]
    fn matrix4f_multiplication() {
        let m1 = Matrix4f::new_from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix4f::new_from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = Matrix4f::new_from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        let result = m1 * m2;
        assert_eq!(result, expected);
    }

    #[test]
    fn matrix4f_vector_multiplication() {
        let m = Matrix4f::new_from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let c = m * b;
        let expected = Vec3::new(18.0, 24.0, 33.0);
        assert_eq!(c, expected);
    }

    #[test]
    fn matrix4f_identity_multiplication() {
        let m1 = Matrix4f::new_from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let i = Matrix4f::identity();
        assert_eq!(m1 * i, m1);
    }

    #[test]
    fn matrix4f_transposition() {
        let m1 = Matrix4f::new_from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let expected = Matrix4f::new_from([
            [1.0, 5.0, 9.0, 5.0],
            [2.0, 6.0, 8.0, 4.0],
            [3.0, 7.0, 7.0, 3.0],
            [4.0, 8.0, 6.0, 2.0],
        ]);
        let t = m1.transpose();
        assert_eq!(t, expected);
    }

    #[test]
    fn matrix4f_identity_transposition() {
        let i = Matrix4f::identity();
        assert_eq!(i.transpose(), i);
    }

    #[test]
    fn matrix4f_submatrix() {
        let m = Matrix4f::identity();
        let i = m.submatrix(3, 3);
        assert_eq!(i, Matrix3f::identity());
    }

    #[test]
    fn matrix4f_determinant() {
        let m = Matrix4f::new_from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn matrix4f_inversion() {
        let m = Matrix4f::new_from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let expected = Matrix4f::new_from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        let iv = m.invert().unwrap();
        assert_eq!(iv, expected);
    }

    #[test]
    fn matrix4f_inversion_undoes_multiplication() {
        let a = Matrix4f::new_from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        let b = Matrix4f::new_from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let c = a * b;
        let ivb = b.invert().unwrap();
        let a2 = c * ivb;

        assert_eq!(a, a2);
    }

    #[test]
    fn matrix4f_identity_inverse_is_identity() {
        let i = Matrix4f::identity();
        assert_eq!(i.invert().unwrap(), i);
    }

    #[test]

    fn matrix4f_multiplication_by_inverse_is_identity() {
        let b = Matrix4f::new_from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let i = b.invert().unwrap() * b;
        assert_eq!(i, Matrix4f::identity());
    }
}
