use crate::primitives::{point::Point, tuple::Tuple};

use super::{
    object::{Intersection, Intersections, Object},
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
    pub fn intersect<'a>(
        &self,
        r: &Ray,
        object: &'a Object,
        i: &mut Intersections<'a>,
    ) -> Vec<Intersection<'a>> {
        let a = r.direction() ^ r.direction();
        let oc = r.origin() - self.center();
        let b = 2.0 * (r.direction() ^ (oc));
        let c = (oc ^ oc) - self.radius() * self.radius();

        let discriminant = b * b - (4.0 * a * c);
        if discriminant >= 0.0 {
            let sqrt_disc = f64::sqrt(discriminant);
            let t1 = (-b - sqrt_disc) / (2.0 * a);
            let t2 = (-b + sqrt_disc) / (2.0 * a);

            let i1 = Intersection::new(t1, object);
            let i2 = Intersection::new(t2, object);

            i.push(i1);
            i.push(i2);

            return vec![i1, i2];
        }

        return vec![];
    }

    pub fn new(center: Point, radius: f64) -> Self {
        Sphere {
            center,
            radius,
            ..Default::default()
        }
    }

    pub fn new_with_transformation(center: Point, radius: f64) -> Self {
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
    use crate::{
        core::transforms::Transformations,
        primitives::{matrix4f::Matrix4f, vec3::Vec3},
    };

    use super::*;

    #[test]
    fn sphere_default_transform_is_identity() {
        let s = Object::sphere();
        assert_eq!(*s.transformation(), Matrix4f::identity());
    }

    #[test]
    fn ray_sphere_intersection() {
        {
            let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Object::sphere();

            let mut i = Intersections::new();

            s.intersect(&r, &mut i);

            assert_eq!(i[0].t, 4.0);
            assert_eq!(i[1].t, 6.0);
        }
        {
            let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Object::sphere();

            let mut i = Intersections::new();

            s.intersect(&r, &mut i);

            assert_eq!(i[0].t, 5.0);
            assert_eq!(i[1].t, 5.0);
        }
        {
            let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Object::sphere();

            let mut i = Intersections::new();

            s.intersect(&r, &mut i);

            assert_eq!(i.len(), 0);
        }
        {
            let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Object::sphere();

            let mut i = Intersections::new();

            s.intersect(&r, &mut i);
            assert_eq!(i[0].t, -1.0);
            assert_eq!(i[1].t, 1.0);
        }
        {
            let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Object::sphere();

            let mut i = Intersections::new();

            s.intersect(&r, &mut i);

            assert_eq!(i[0].t, -6.0);
            assert_eq!(i[1].t, -4.0);
        }
    }

    #[test]
    fn can_change_sphere_transformation() {
        let mut s = Object::sphere();
        let t = Transformations::translate(1.0, 2.0, 3.0);

        s.set_transformation(t);

        assert_eq!(*s.transformation(), t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let mut s = Object::sphere();
        let mut i = Intersections::new();

        let t = Transformations::scale(2.0, 2.0, 2.0);

        s.set_transformation(t);
        s.intersect(&r, &mut i);

        assert_eq!(i.len(), 2);
        assert_eq!(i[0].t, 3.0);
        assert_eq!(i[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let mut s = Object::sphere();
        let mut i = Intersections::new();

        s.set_transformation(Transformations::translate(5.0, 0.0, 0.0));
        s.intersect(&r, &mut i);

        assert_eq!(i.len(), 0);
    }
}
