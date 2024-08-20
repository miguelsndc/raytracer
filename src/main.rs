// use std::f64::consts::PI;

// use the_ray_tracer_challenge::{
//     core::{
//         canvas::Canvas,
//         light::{lighting, PointLight},
//         object::{Intersections, Object},
//         ray::Ray,
//         transforms::{Transform, Transformations},
//     },
//     primitives::{color::Color, matrix4f::Matrix4f, point::Point, tuple::Tuple},
// };

// fn main() {
//     let ray_origin = Point::new(0.0, 0.0, -5.0);
//     let wall_z = 5.0;
//     let wall_size = 7.0;
//     const CANVAS_HEIGHT: usize = 1000;
//     const CANVAS_WIDTH: usize = 1000;

//     let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
//     let half = wall_size / 2.0;
//     let pixel_size = wall_size / (CANVAS_HEIGHT as f64);

//     let mut sphere = Object::sphere();
//     let light_source = PointLight::new(Point::new(-10.0, 10.0, 10.0), Color::new(0.8, 0.8, 0.9));

//     // let m = Matrix4f::identity()
//     //     .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
//     //     .rotate_y(PI / 4.0)
//     //     .transform();

//     // sphere.set_transformation(m);
//     sphere.set_material_color(Color::new(2.0, 0.3, 1.0));

//     let mut i = Intersections::new();

//     for y in 0..CANVAS_HEIGHT {
//         let world_y = (half as f64) - pixel_size * (y as f64);
//         for x in 0..CANVAS_WIDTH {
//             let world_x = (-half as f64) + pixel_size * (x as f64);
//             let position = Point::new(world_x, world_y, wall_z);
//             let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

//             let intersections = sphere.intersect(&ray);

//             if !intersections.ok {
//                 continue;
//             }

//             let hit = intersections.i.iter().max().unwrap();

//             if hit.t >= 0.0 {
//                 let point = ray.position(hit.t);
//                 let normal = hit.object.normal_at(point);
//                 let eye = ray.direction();

//                 let color = lighting(hit.object.material(), &light_source, point, eye, normal);
//                 canvas.draw_pixel(x, y, color);
//             }
//         }
//     }

//     let _ = canvas.export_to_ppm("output.ppm");
// }

use the_ray_tracer_challenge::{
    core::{light::Material, ray::Ray, world::World},
    primitives::{point::Point, tuple::Tuple, vec3::Vec3},
};

fn main() {}
