use the_ray_tracer_challenge::core::object::{Intersections, Object};
use the_ray_tracer_challenge::core::ray::Ray;
use the_ray_tracer_challenge::core::transforms::Transformations;
use the_ray_tracer_challenge::core::{canvas::Canvas, transforms::Transform};
use the_ray_tracer_challenge::primitives::{color::Color, point::Point, tuple::Tuple, vec3::Vec3};

fn main() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
    let mut s = Object::sphere();
    let mut i = Intersections::new();

    s.set_transformation(Transformations::scale(2.0, 2.0, 2.0));
    s.intersect(&r, &mut i);

    assert_eq!(i.len(), 2);
    assert_eq!(i[0].t, 3.0);
    assert_eq!(i[1].t, 7.0);
}
