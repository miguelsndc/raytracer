const MATRIX_SIZE: usize = 4;

#[derive(Clone, Copy, Debug)]
pub struct Matrix4f {
    inner: [f64; MATRIX_SIZE * MATRIX_SIZE],
}

impl Matrix4f {
    fn new() -> Self {
        Self {
            inner: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }

    fn identity() -> Self {
        Self {
            inner: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl PartialEq for Matrix4f {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                if self.inner[i * MATRIX_SIZE + j] != other.inner[i * MATRIX_SIZE + j] {
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
