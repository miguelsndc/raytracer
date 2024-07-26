use crate::primitives::{point::Point, tuple::Tuple};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        return Self {
            center: Point::zero(),
            radius: 1.0,
        };
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn radius(&self) -> f64 {
        return self.radius;
    }

    pub fn center(&self) -> Point {
        return self.center;
    }
}
