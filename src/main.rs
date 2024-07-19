pub mod vector;
pub mod color;
pub mod canvas;

use color::color::Color;
use vector::vec4::{Vec4,normalize};
use crate::canvas::canvas::Canvas;

struct Projectile {
    position: Vec4,
    velocity: Vec4,
}

struct Environment {
    gravity: Vec4,
    wind: Vec4,
}

const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;

fn tick(env: &mut Environment, proj: &mut Projectile) {
    proj.position += proj.velocity;
    proj.velocity += env.gravity + env.wind;
}

fn main() {
    let mut p = Projectile {
        position: Vec4::new(0.0, 0.0, 0.0),
        velocity: normalize(&Vec4::new(1.0, 1.8, 0.0)) * 11.25,
    };

    let mut e = Environment {
        gravity: Vec4::new(0.0, -0.098, 0.0),
        wind: Vec4::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    
    let color = Color {
        r: 0.4,
        g: 0.3,
        b: 0.8,
    };

    while p.position.x < WIDTH as f32 {
        tick(&mut e, &mut p);
        canvas.write_pixel(p.position.x, p.position.y, color);
    }

    match canvas.to_ppm() {
        Ok(_) => println!("ok"),
        Err(e) => eprintln!("Error writing to ppm: {}", e),
    };
}
