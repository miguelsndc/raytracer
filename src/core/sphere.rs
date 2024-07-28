use crate::primitives::{point::Point, tuple::Tuple};

use super::{
    object::{self, Intersection, IntersectionPush, Intersections, Object, Shape},
    ray::Ray,
};

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn intersect<'a>(&self, r: &Ray, object: &'a Object, i: &mut Intersections<'a>) {
        let a = r.direction() ^ r.direction();
        let oc = self.center() - r.origin();
        let b = -2.0 * (r.direction() ^ (oc));
        let c = (oc ^ oc) - self.radius() * self.radius();

        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b - f64::sqrt(discriminant)) / 2.0 * a;
            let t2 = (-b + f64::sqrt(discriminant)) / 2.0 * a;

            i.push(Intersection::new(t1, object));
            i.push(Intersection::new(t2, object));
        }
    }

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

#[cfg(test)]
mod tests {
    use super::Sphere;

    #[test]
    fn intersection_encapsulates_t_and_object() {}
}
