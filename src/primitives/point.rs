use super::{float::ApproxEq, tuple::Tuple, vec3::Vec3};
#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Point { x, y, z };
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
        1.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x.approx_eq_low_precision(other.x)
            && self.y.approx_eq_low_precision(other.y)
            && self.z.approx_eq_low_precision(other.z);
    }
}

impl std::ops::Add<Vec3> for Point {
    type Output = Point;
    fn add(self, rhs: Vec3) -> Self {
        Self {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vec3::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z());
    }
}
