use super::float::ApproxEq;

const MATRIX_SIZE: usize = 2;
#[derive(Clone, Copy, Debug)]
pub struct Matrix2f {
    inner: [f64; MATRIX_SIZE * MATRIX_SIZE],
}

impl Matrix2f {
    pub fn new() -> Self {
        Self {
            inner: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }

    pub fn identity() -> Self {
        Self {
            inner: [1.0, 0.0, 0.0, 1.0],
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
        let mut m = Matrix2f::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                m[(j, i)] = self[(i, j)];
            }
        }
        return m;
    }

    pub fn determinant(&self) -> f64 {
        return self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)];
    }
}
impl PartialEq for Matrix2f {
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

impl std::ops::Index<(usize, usize)> for Matrix2f {
    type Output = f64;
    fn index(&self, indexes: (usize, usize)) -> &f64 {
        let (row, col) = indexes;
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        return &self.inner[row * MATRIX_SIZE + col];
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix2f {
    fn index_mut(&mut self, indexes: (usize, usize)) -> &mut f64 {
        let (row, col) = indexes;
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        return &mut self.inner[row * MATRIX_SIZE + col];
    }
}

impl std::ops::Mul for Matrix2f {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut m = Matrix2f::new();
        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                m[(row, col)] = self[(row, 0)] * rhs[(0, col)] + self[(row, 1)] * rhs[(1, col)]
            }
        }
        return m;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix2f_initialization() {
        let m = Matrix2f::new_from([[1.0, 2.0], [3.0, 4.0]]);

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(1, 0)], 3.0);
        assert_eq!(m[(1, 1)], 4.0);
    }

    #[test]
    fn matrix2f_identity() {
        let m = Matrix2f::identity();

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
    fn matrix2f_determinant() {
        let m = Matrix2f::new_from([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(m.determinant(), 17.0);
    }
}
