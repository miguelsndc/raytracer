use the_ray_tracer_challenge::{
    core::{
        canvas::Canvas,
        object::{Intersections, Object},
        ray::Ray,
        transforms::Transformations,
    },
    primitives::{color::Color, point::Point, tuple::Tuple},
};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -4.0);
    let circle_color = Color::red();
    let wall_z = 4.0;
    let wall_size = 7.0;
    const CANVAS_HEIGHT: usize = 256;
    const CANVAS_WIDTH: usize = 256;

    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let half = wall_size / 2.0;
    let pixel_size = wall_size / (CANVAS_HEIGHT as f64);

    let mut sphere = Object::sphere();
    let t = Transformations::shear(1.0, 1.2, 0.5, 0.0, 0.0, 0.0);
    sphere.set_transformation(t);
    let mut i = Intersections::new();

    for y in 0..CANVAS_HEIGHT {
        let world_y = (half as f64) - pixel_size * (y as f64);
        for x in 0..CANVAS_WIDTH {
            let world_x = (-half as f64) + pixel_size * (x as f64);
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            if x == 50 && y == 50 {
                println!("half");
            }

            let intersect = sphere.intersect(&ray, &mut i);

            if intersect.len() > 0 {
                for it in intersect.iter() {
                    if it.t > 0.0 {
                        canvas.draw_pixel(x, y, circle_color);
                    }
                }
            }
        }
    }

    let _ = canvas.export_to_ppm("output.ppm");
}
