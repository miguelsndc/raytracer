pub mod vector;
use crate::vector::vec4::{cross, dot, magnitude, normalize, Vec4};

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

struct Projectile {
    position: Point,
    velocity: Vec4,
}

struct Env {
    gravity: Vec4,
    wind: Vec4,
}

fn main() {
    println!("Hello, world!");
    
}
