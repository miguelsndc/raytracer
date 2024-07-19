pub mod canvas {
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
            if x < self.width as f32 && x >= 0. && y < self.height as f32 && y >= 0. {
                let index = (y as i32 * self.height + x as i32) as usize;
                self.grid[index] = color;
            }
        }

        pub fn pixel_at(&self, i: i32, j: i32) -> Color {
            let i = (j * self.height + i) as usize;
            return self.grid[i];
        }

        pub fn to_ppm(&self) -> Result<(), std::io::Error> {
            let path = Path::new("output.ppm");
            let file = File::create(&path)?;
            let mut file = BufWriter::new(file);

            writeln!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

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
                    writeln!(file, "{} {} {}", r, g, b)?;
                }
            }
            file.flush()?;
            Ok(())
        }
    }
}
