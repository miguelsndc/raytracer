use super::{ray::Ray, sphere::Sphere};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect<'a>(&self, ray: &Ray, object: &'a Object, p: &mut Intersections<'a>) {
        match self {
            Shape::Sphere(s) => s.intersect(ray, object, p),
        }
    }
}

pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Intersection<'a> {
        return Intersection { t, object };
    }
}

pub struct Intersections<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn push(&mut self, intersection: Intersection<'a>) {
        self.intersections.push(intersection);
    }

    pub fn new() -> Self {
        Intersections {
            intersections: Vec::new(),
        }
    }
}

impl<'a> std::ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.intersections[index];
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Object {
    shape: Shape,
}

impl Object {
    pub fn sphere() -> Self {
        let shape = Shape::Sphere(Sphere::default());
        return Object { shape };
    }

    pub fn shape(&self) -> Shape {
        return self.shape;
    }
}

pub trait IntersectionPush<'a> {
    fn t(&mut self, t: f64);
}

#[cfg(test)]
mod tests {
    use crate::primitives::{point::Point, tuple::Tuple, vec3::Vec3};

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let shape = Object::sphere();
        let i = Intersection::new(3.5, &shape);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &shape);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Object::sphere();

        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let mut intersections = Intersections::new();

        intersections.push(i1);
        intersections.push(i2);

        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[1].t, 2.0);
    }

    #[test]
    fn intersection() {
        let s = Object::sphere();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        let mut intersections = Intersections::new();

        s.shape().intersect(&ray, &s, &mut intersections);

        assert_eq!(intersections[0].object, &s);
        assert_eq!(intersections[1].object, &s);
    }
}
