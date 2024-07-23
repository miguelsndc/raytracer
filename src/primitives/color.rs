use super::float::ApproxEq;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        return self.r.approx_eq_low_precision(other.r)
            && self.g.approx_eq_low_precision(other.g)
            && self.b.approx_eq_low_precision(other.b);
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }

    pub fn red() -> Color {
        return Color::new(1.0, 0.0, 0.0);
    }

    pub fn green() -> Color {
        return Color::new(0.0, 1.0, 0.0);
    }

    pub fn blue() -> Color {
        return Color::new(0.0, 0.0, 1.0);
    }

    pub fn white() -> Color {
        return Color::new(1.0, 1.0, 1.0);
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
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

// Hadamard Product
impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_initialization() {
        let c1 = Color::new(1.0, 0.0, 0.0);

        assert_eq!(c1.r, 1.0);
        assert_eq!(c1.g, 0.0);
        assert_eq!(c1.b, 0.0);
    }

    #[test]
    fn color_primitives() {
        let r = Color::red();
        let g = Color::green();
        let b = Color::blue();
        let w = Color::white();
        let black = Color::black();

        assert_eq!(r, Color::new(1.0, 0.0, 0.0));
        assert_eq!(g, Color::new(0.0, 1.0, 0.0));
        assert_eq!(b, Color::new(0.0, 0.0, 1.0));
        assert_eq!(w, Color::new(1.0, 1.0, 1.0));
        assert_eq!(black, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_addition() {
        let c1 = Color::new(0.4, 0.3, 0.2);
        let c2 = Color::new(0.4, 0.3, 0.2);
        let expected = Color::new(0.8, 0.6, 0.4);

        assert_eq!(c1 + c2, expected);
    }

    #[test]
    fn color_subtraction() {
        let c1 = Color::new(0.5, 0.7, 0.2);
        let c2 = Color::new(0.4, 0.3, 0.2);
        let expected = Color::new(0.1, 0.4, 0.0);

        assert_eq!(c1 - c2, expected);
    }

    #[test]
    fn color_scaling() {
        let c1 = Color::new(0.5, 0.7, 0.2);
        let expected = Color::new(0.25, 0.35, 0.1);
        assert_eq!(c1 * 0.5, expected);
    }

    #[test]
    fn color_blending() {
        let c1 = Color::new(0.5, 0.7, 0.2);
        let c2 = Color::new(0.25, 0.35, 0.1);
        let expected = Color::new(0.125, 0.245, 0.02);
        assert_eq!(c1 * c2, expected);
    }
}
