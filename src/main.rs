use core::f64;

use the_ray_tracer_challenge::core::{canvas::Canvas, transforms::Transform};
use the_ray_tracer_challenge::primitives::{color::Color, vec3::Vec3, tuple::Tuple,point::Point};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
fn main() {
    let mut c = Canvas::new(WIDTH, HEIGHT);
    let orig = Point::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0, 1.0);
    c[orig.y() as usize][orig.x() as usize] = Color::blue();
    let p = orig.translate(30.0, 30.0, 1.0).transform();
    let mut v = Vec3::new(p.x() - 100.0, p.y() - 100.0, p.z());
    c[(orig.y() + v.y()) as usize][(orig.x() + v.x()) as usize] = Color::red();
    for _ in 0..64 {
        v = v.rotate_z(f64::consts::PI / 32.0).transform();
        c[(orig.y() + v.y()) as usize][(orig.x() + v.x()) as usize] = Color::green();
    }
    let _ = c.export_to_ppm("output.ppm");
}
