pub mod vec4 {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Vec4 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }

    impl Default for Vec4 {
        fn default() -> Self {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }
        }
    }

    pub fn dot(v1: &Vec4, v2: &Vec4) -> f32 {
        return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w;
    }

    pub fn cross(v1: &Vec4, v2: &Vec4) -> Vec4 {
        Vec4 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
            w: 0.0,
        }
    }

    pub fn magnitude(v: &Vec4) -> f32 {
        return f32::sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
    }

    pub fn normalize(v: &Vec4) -> Vec4 {
        let mag = magnitude(v);
        Vec4 {
            x: v.x / mag,
            y: v.y / mag,
            z: v.z / mag,
            w: 0.0,
        }
    }

    
    impl Vec4 {
        pub fn magnitude(&self) -> f32 {
            return f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        }

        pub fn normalize(&self) -> Self {
            let mag = self.magnitude();
            return (*self) / mag;
        }

        pub fn dot(&self, other: &Vec4) -> f32 {
            return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
        }
        
        pub fn cross(&self, other: &Vec4) -> Vec4 {
            Vec4 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
                w: 0.0,
            }
        }

        pub fn new(x: f32, y: f32, z: f32) -> Vec4 {
                Vec4 {
                    x,
                    y,
                    z,
                    w: 1.0,
                }
            }
    }

    // operators

    impl Neg for Vec4 {
        type Output = Self;
        fn neg(self) -> Self {
            Self {
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: -self.w,
            }
        }
    }

    impl Add for Vec4 {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
                w: self.w + other.w,
            }
        }
    }

    impl AddAssign for Vec4 {
        fn add_assign(&mut self, rhs: Self) {
            *self = Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }

    impl Sub for Vec4 {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
                w: self.w - other.w,
            }
        }
    }

    impl SubAssign for Vec4 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
                w: self.w - rhs.w,
            }
        }
    }

    impl Div<f32> for Vec4 {
        type Output = Vec4;
        fn div(self, rhs: f32) -> Self::Output {
            Self {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
                w: self.w / rhs,
            }
        }
    }

    impl DivAssign<f32> for Vec4 {
        fn div_assign(&mut self, rhs: f32) {
            *self = Self {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
                w: self.w / rhs,
            }
        }
    }

    impl Mul<f32> for Vec4 {
        type Output = Vec4;
        fn mul(self, rhs: f32) -> Self::Output {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
                w: self.w * rhs,
            }
        }
    }

    impl MulAssign<f32> for Vec4 {
        fn mul_assign(&mut self, rhs: f32) {
            *self = Self {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
                w: self.w * rhs,
            }
        }
    }
}
