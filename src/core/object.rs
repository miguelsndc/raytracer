use crate::primitives::{color::Color, matrix4f::Matrix4f, point::Point, tuple::Tuple, vec3::Vec3};

use super::{light::Material, ray::Ray, sphere::Sphere, transforms::Transform};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect<'a>(
        &self,
        ray: &Ray,
        object: &'a Object,
        p: &mut Intersections<'a>,
    ) -> Vec<Intersection<'a>> {
        match self {
            Shape::Sphere(s) => s.intersect(ray, object, p),
        }
    }

    pub fn normal_at(&self, p: Point) -> Vec3 {
        match self {
            Shape::Sphere(s) => s.normal_at(p),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Intersection<'a> {
        return Intersection { t, object };
    }
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.t.is_nan() {
            return std::cmp::Ordering::Less;
        } else if other.t.is_nan() {
            return std::cmp::Ordering::Greater;
        } else if self.t < other.t {
            return std::cmp::Ordering::Less;
        } else if self.t == other.t {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Greater;
        }
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

    pub fn len(&self) -> usize {
        return self.intersections.len();
    }

    // intersection with the lowest nonnegative t value
    pub fn hit(&self) -> Option<&Intersection<'a>> {
        return self.intersections.iter().find(|i| i.t >= 0.0);
    }

    pub fn from_intersections(from: Vec<Intersection<'a>>) -> Intersections<'a> {
        let mut container = Intersections::new();

        for i in from {
            container.push(i);
        }

        return container;
    }

    pub fn sort(&mut self) {
        self.intersections.sort_unstable();
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
    transformation: Matrix4f,
    transformation_inverse: Matrix4f,
    transformation_inverse_transposed: Matrix4f,
    material: Material,
}

impl Object {
    pub fn sphere() -> Self {
        let shape = Shape::Sphere(Sphere::default());
        return Object {
            shape,
            transformation: Matrix4f::identity(),
            transformation_inverse: Matrix4f::identity(),
            transformation_inverse_transposed: Matrix4f::identity(),
            material: Material::default(),
        };
    }

    pub fn set_transformation(&mut self, transformation: Matrix4f) {
        self.transformation = transformation;
        self.transformation_inverse = transformation.invert().unwrap_or(Matrix4f::identity());
        self.transformation_inverse_transposed = self.transformation_inverse.transpose();
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn transformation(&self) -> &Matrix4f {
        return &self.transformation;
    }

    pub fn material(&self) -> Material {
        return self.material;
    }

    pub fn set_material_color(&mut self, color: Color) {
        self.material.color = color;
    }

    pub fn intersect<'a>(&'a self, ray: &Ray, i: &mut Intersections<'a>) -> Vec<Intersection<'a>> {
        let r = (*ray).transform(&self.transformation_inverse);
        return self.shape.intersect(&r, self, i);
    }

    pub fn normal_at(&self, point: Point) -> Vec3 {
        let obj_point = self.transformation_inverse * point;
        let obj_normal = self.shape.normal_at(obj_point);
        let world_normal = self.transformation_inverse_transposed * obj_normal;
        return Vec3::new(world_normal.x(), world_normal.y(), world_normal.z()).normalize();
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

    #[test]
    fn hit_with_positive_t_only() {
        let s = Object::sphere();

        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let i = Intersections::from_intersections(vec![i1, i2]);

        assert_eq!(i.hit().unwrap(), &i1);
    }

    #[test]
    fn hit_with_mixed_t_values() {
        let s = Object::sphere();

        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);

        let i = Intersections::from_intersections(vec![i1, i2]);

        assert_eq!(i.hit().unwrap(), &i2);
    }

    #[test]
    fn hit_with_negative_t_values() {
        let s = Object::sphere();

        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-2.0, &s);

        let i = Intersections::from_intersections(vec![i1, i2]);

        assert!(i.hit().is_none());
    }

    #[test]
    fn hit_is_always_the_first_nonnegatievable_intersection() {
        let s = Object::sphere();

        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let mut intersections = Intersections::from_intersections(vec![i1, i2, i3, i4]);

        intersections.sort();

        assert_eq!(intersections.hit(), Some(&i4));
    }
}
