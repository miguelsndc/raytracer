pub const EPSILON: f64 = 1.0e-7;
pub const LOW_EPSILON: f64 = 1.0e-3;

use float_cmp::approx_eq;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, other: Rhs) -> bool;
    fn approx_eq_low_precision(self, other: Rhs) -> bool;
    fn approx_eq_epsilon(self, other: Rhs, epsilon: f64) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        return self.approx_eq_epsilon(other, EPSILON);
    }

    fn approx_eq_low_precision(self, other: Self) -> bool {
        return self.approx_eq_epsilon(other, LOW_EPSILON);
    }

    fn approx_eq_epsilon(self, other: Self, epsilon: f64) -> bool {
        return approx_eq!(f64, self, other, epsilon = epsilon);
    }
}
