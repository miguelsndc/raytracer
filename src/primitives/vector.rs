use super::{float::ApproxEq, tuple::Tuple};
use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Vector { x, y, z };
    }

    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        return f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    pub fn normalize(&self) -> Self {
        return (*self) / self.magnitude();
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        return self.x.approx_eq_low_precision(other.x)
            && self.y.approx_eq_low_precision(other.y)
            && self.z.approx_eq_low_precision(other.z);
    }
}
// Operators --------------------------------

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// Implements Cross product, non-commutative.
impl Mul for Vector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

// Implements dot product
impl BitXor for Vector {
    type Output = f64;
    fn bitxor(self, rhs: Self) -> f64 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_four_operations() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        let vadd = v1 + v2;
        let vsub = v1 - v2;
        let vmul = v1 * 2.0;
        let vdiv = v1 / 2.0;

        assert_eq!(vadd, Vector::new(3.0, 5.0, 7.0));
        assert_eq!(vsub, Vector::new(-1.0, -1.0, -1.0));
        assert_eq!(vmul, Vector::new(2.0, 4.0, 6.0));
        assert_eq!(vdiv, Vector::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let dot = v1 ^ v2;

        assert_eq!(dot, 20.0);
    }

    #[test]
    fn magnitude() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v1.magnitude(), 1.0);
    }

    #[test]
    fn normalization() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v1.normalize(), v1 / v1.magnitude());
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let normal_v1 = v1.normalize();

        assert_eq!(normal_v1.magnitude(), 1.0);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        let cross = v1 * v2;
        let cross_rev = v2 * v1;

        assert_eq!(cross, Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(cross_rev, Vector::new(1.0, -2.0, 1.0));
        assert_eq!(cross ^ v1, 0.0);
        assert_eq!(cross ^ v2, 0.0);
    }
}
