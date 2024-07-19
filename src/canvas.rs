pub mod canvas {
    use core::panic;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;

    use crate::color::color::Color;
    pub struct Canvas {
        pub width: i32,
        pub height: i32,
        pub grid: Vec<Color>,
    }

    impl Canvas {
        pub fn new(width: i32, height: i32) -> Canvas {
            let size = (width * height) as usize;
            Self {
                width,
                height,
                grid: vec![Color::default(); size],
            }
        }

        pub fn write_pixel(&mut self, x: f32, y: f32, color: Color) {
            println!("{}, {}", x, y);
            let index = (y as i32 * self.height + y as i32) as usize;
            println!("i: {}", index);
            if x < self.width as f32 && x >= 0. && y < self.height as f32 && y >= 0. {
                self.grid[index] = color;
            }
        }

        pub fn pixel_at(&self, i: i32, j: i32) -> Color {
            let i = (i * self.height + j) as usize;
            return self.grid[i];
        }

        pub fn to_ppm(&self) -> Result<(), std::io::Error> {
            let path = Path::new("output.ppm");
            let display = path.display();

            let file = match File::create(&path) {
                Ok(file) => file,
                Err(why) => panic!("Couldn't open ppm file {}: {}", display, why),
            };

            let mut file = BufWriter::new(file);
            let header = format!("P3\n{} {}\n255\n", self.width, self.height);

            file.write_all(header.as_bytes())?;

            for i in 0..self.height {
                for j in 0..self.width {
                    let offset = i * self.height + j;
                    let color = self.grid.get(offset as usize).unwrap_or(&Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    });
                    let r = (color.r * 255.0) as u8;
                    let g = (color.g * 255.0) as u8;
                    let b = (color.b * 255.0) as u8;
                    let s = format!("{} {} {}\n", r, g, b);

                    file.write_all(s.as_bytes())?;
                }
            }

            file.flush()?;

            Ok(())
        }
    }
}
