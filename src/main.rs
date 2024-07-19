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

fn in_bounds(v: &Projectile, low: f32, high: f32) -> bool {
    return v.position.x >=low && v.position.x < high && v.position.y >= low && v.position.y < high;
}


fn main() {
    let mut p = Projectile {
        position: Vec4::new(0.0, (HEIGHT - 1) as f32 , 0.0),
        velocity: normalize(&Vec4::new(1.0, -9.0, 0.0)) * 6.0,
    };

    let mut e = Environment {
        gravity: Vec4::new(0.0, -0.1, 0.0),
        wind: Vec4::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    
    let color = Color {
        r: 0.4,
        g: 0.3,
        b: 0.8,
    };


    while in_bounds(&p, 0.0, 255.0){
        tick(&mut e,&mut p);
        canvas.write_pixel(p.position.x, p.position.y, color);
    }
    let _ =    canvas.to_ppm();
}
