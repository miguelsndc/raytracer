use crate::primitives::{float::ApproxEq, matrix2f::Matrix2f};

const MATRIX_SIZE: usize = 3;
#[derive(Clone, Copy, Debug)]
pub struct Matrix3f {
    inner: [f64; MATRIX_SIZE * MATRIX_SIZE],
}

impl Matrix3f {
    pub fn new() -> Self {
        Self {
            inner: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }

    pub fn identity() -> Self {
        Self {
            inner: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
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
        let mut m = Matrix3f::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                m[(j, i)] = self[(i, j)];
            }
        }
        return m;
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2f {
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);

        let mut m = Matrix2f::new();
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
            + self.cofactor(0, 2) * self[(0, 2)];
        return det;
    }
}
impl PartialEq for Matrix3f {
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

impl std::ops::Index<(usize, usize)> for Matrix3f {
    type Output = f64;
    fn index(&self, indexes: (usize, usize)) -> &f64 {
        let (row, col) = indexes;
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        return &self.inner[row * MATRIX_SIZE + col];
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix3f {
    fn index_mut(&mut self, indexes: (usize, usize)) -> &mut f64 {
        let (row, col) = indexes;
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        return &mut self.inner[row * MATRIX_SIZE + col];
    }
}

impl std::ops::Mul for Matrix3f {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut m = Matrix3f::new();
        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                m[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)];
            }
        }
        return m;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix3f_initialization() {
        let m = Matrix3f::new_from([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [6.0, 7.0, 8.0]]);

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(1, 1)], 4.0);
        assert_eq!(m[(2, 2)], 8.0);
    }

    #[test]
    fn matrix3f_identity() {
        let m = Matrix3f::identity();

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
    fn matrix3f_submatrix() {
        let m = Matrix3f::identity();
        let i = m.submatrix(2, 2);
        assert_eq!(i, Matrix2f::identity());
    }
    #[test]
    fn matrix3f_minor() {
        let m = Matrix3f::new_from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let d = m.minor(1, 0);
        assert_eq!(d, 25.0);
    }

    #[test]
    fn matrix3f_cofactor() {
        let m = Matrix3f::new_from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let minor = m.minor(0, 0);
        let minor2 = m.minor(1, 0);

        assert_eq!(m.cofactor(0, 0), minor); // even pos
        assert_eq!(m.cofactor(1, 0), -minor2); // odd pos;
    }
    #[test]
    fn matrix3f_determinant() {
        let m = Matrix3f::new_from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(m.determinant(), -196.0);
    }
}
