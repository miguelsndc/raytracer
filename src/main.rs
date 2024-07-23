use std::fmt::Error;

use the_ray_tracer_challenge::canvas::Canvas;
use the_ray_tracer_challenge::primitives::{color::Color, point::Point, tuple::Tuple, vec3::Vec3};

pub struct Projectile {
    position: Point,
    velocity: Vec3,
}

pub struct Env {
    gravity: Vec3,
    wind: Vec3,
}

fn tick(proj: &mut Projectile, env: &mut Env) {
    proj.position = proj.position + proj.velocity;
    proj.velocity += env.gravity + env.wind;
}

fn within_bounds(x: f64, y: f64, w: f64, h: f64) -> bool {
    return x >= 0.0 && x < w && y >= 0.0 && y < h;
}

fn main() {
    let mut c = Canvas::new(256, 256);
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vec3::new(1.0, 1.8, 0.0).normalize() * 6.0;
    let gravity = Vec3::new(0.0, -0.1, 0.0);
    let wind = Vec3::new(-0.01, 0.0, 0.0);

    let mut p = Projectile {
        position: start,
        velocity,
    };

    let mut e = Env { gravity, wind };

    loop {
        tick(&mut p, &mut e);

        let x = p.position.x();
        let y = p.position.y();

        if !within_bounds(x, y, 256.0, 256.0) {
            break;
        }

        c[x as usize][y as usize] = Color::green();
    }

    match c.export_to_ppm("output.ppm") {
        Ok(_) => println!("Ok"),
        Err(e) => println!("Error: {}", e),
    }
}
