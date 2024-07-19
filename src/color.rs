pub mod color {
    use std::ops::{Add, AddAssign,Sub, SubAssign, Mul, MulAssign};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
    }

    impl Default for Color {
        fn default() -> Self {
            Color { r: 0.0, g: 0.0, b: 0.0 }
        }
    }

    impl Add for Color {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            Self {
                r: self.r + other.r,
                g: self.g + other.g,
                b: self.b + other.b,
            }
        }
    }

    impl AddAssign for Color {
        fn add_assign(&mut self, rhs: Self) {
            *self = Self {
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
            }
        }
    }

    impl Sub for Color {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            Self {
                r: self.r - other.r,
                g: self.g - other.g,
                b: self.b - other.b,
            }
        }
    }

    impl SubAssign for Color {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Self {
                r: self.r - rhs.r,
                g: self.g - rhs.g,
                b: self.b - rhs.b,
            }
        }
    }


    impl Mul<f32> for Color {
        type Output = Color;
        fn mul(self, rhs: f32) -> Self::Output {
            Self {
                r: self.r * rhs,
                g: self.g * rhs,
                b: self.b * rhs,
            }
        }
    }

    impl MulAssign<f32> for Color {
        fn mul_assign(&mut self, rhs: f32) {
            *self = Self {
                r: self.r * rhs,
                g: self.g * rhs,
                b: self.b * rhs,
            }
        }
    }
}