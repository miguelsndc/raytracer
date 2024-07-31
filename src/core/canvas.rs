use std::{fs::File, io::Write, path::Path};

use crate::primitives::color::*;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn export_to_ppm(&self, path: &'static str) -> Result<(), std::io::Error> {
        let p = Path::new(path);
        let mut file = File::create(p)?;
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write_all(header.as_bytes())?;
        for pixel in self.pixels.iter() {
            let (r, g, b) = clamp_color(pixel);
            let line = format!("{} {} {}\n", r, g, b);
            file.write_all(line.as_bytes())?;
        }
        file.flush()?;
        Ok(())
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if y < self.height && x < self.width {
            self[y as usize][x as usize] = color;
        }
    }
}

fn clamp_color(color: &Color) -> (u8, u8, u8) {
    let r = f64::max(0.0, f64::min(color.r, 1.0));
    let g = f64::max(0.0, f64::min(color.g, 1.0));
    let b = f64::max(0.0, f64::min(color.b, 1.0));
    return ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
}

impl std::ops::Index<usize> for Canvas {
    type Output = [Color];
    fn index(&self, row: usize) -> &[Color] {
        let offset = row * self.width;
        return &self.pixels[offset..offset + self.width];
    }
}

impl std::ops::IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let offset = row * self.width;
        return &mut self.pixels[offset..offset + self.width];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const W: usize = 10;
    const H: usize = 20;
    #[test]
    fn canvas_initialization() {
        let c = Canvas::new(W, H);

        assert_eq!(c.width, W);
        assert_eq!(c.height, 20);

        for pixel in c.pixels.iter() {
            assert_eq!(*pixel, Color::black());
        }
    }

    #[test]
    fn canvas_pixel_io() {
        let mut c = Canvas::new(W, H);
        c[2][3] = Color::black();
        assert_eq!(c[2][3], Color::black());
    }
}
