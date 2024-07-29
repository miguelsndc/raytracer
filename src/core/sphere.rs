use crate::primitives::{matrix4f::Matrix4f, point::Point, tuple::Tuple};

use super::{
    object::{Intersection, Intersections, Object},
    ray::Ray,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f64,
    transformation: Matrix4f,
}

impl Default for Sphere {
    fn default() -> Self {
        return Self {
            center: Point::zero(),
            radius: 1.0,
            transformation: Matrix4f::identity(),
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
        Sphere {
            center,
            radius,
            ..Default::default()
        }
    }

    pub fn transformation(&self) -> Matrix4f {
        return self.transformation;
    }

    pub fn set_transformation(&mut self, transformation: Matrix4f) {
        self.transformation = transformation;
    }

    pub fn new_with_transformation(center: Point, radius: f64, transformation: Matrix4f) -> Self {
        Sphere {
            center,
            radius,
            transformation,
        }
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
    use crate::core::transforms::Transformations;

    use super::*;

    #[test]
    fn sphere_default_transform_is_identity() {
        let s = Sphere::default();
        assert_eq!(s.transformation(), Matrix4f::identity())
    }

    #[test]
    fn can_change_sphere_transformation() {
        let mut s = Sphere::default();
        let t = Transformations::translate(1.0, 2.0, 3.0);

        s.set_transformation(t);

        assert_eq!(s.transformation(), t);
    }
}
