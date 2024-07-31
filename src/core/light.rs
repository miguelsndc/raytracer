use crate::primitives::{color::Color, point::Point, vec3::Vec3};

pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        return PointLight {
            position,
            intensity,
        };
    }

    pub fn intensity(&self) -> Color {
        return self.intensity;
    }

    pub fn position(&self) -> Point {
        return self.position;
    }
}

// phong shading algorithm
pub fn lighting(
    m: Material,
    light: &PointLight,
    position: Point,
    eyev: Vec3,
    normalv: Vec3,
) -> Color {
    let mut diffuse = Color::black();
    let mut ambient = Color::black();
    let mut specular = Color::black();

    let effective_color = m.color * light.intensity();
    let lightv = (light.position - position).normalize();
    ambient = effective_color * m.ambient;

    let light_dot_normal = lightv ^ normalv;
    if light_dot_normal < 0.0 {
        diffuse = Color::black();
        specular = Color::black();
    } else {
        diffuse = effective_color * m.diffuse * light_dot_normal;
        let reflectv = Vec3::reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv ^ eyev;

        if reflect_dot_eye <= 0.0 {
            specular = Color::black();
        } else {
            let factor = reflect_dot_eye.powf(m.shininess);
            specular = light.intensity() * m.specular * factor;
        }
    }

    return ambient + diffuse + specular;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::primitives::tuple::Tuple;

    use super::*;

    #[test]
    fn point_light_initialization() {
        let position = Point::new(0.0, 0.0, 0.0);
        let intensity = Color::new(1.0, 1.0, 1.0);

        let pl = PointLight::new(position, intensity);

        assert_eq!(pl.intensity(), intensity);
        assert_eq!(pl.position(), position);
    }

    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[allow(dead_code)]
    fn setup() -> (Material, Point) {
        return (Material::default(), Point::zero());
    }

    #[test]
    fn light_with_eye_between_normal_and_source() {
        let (m, p) = setup();

        let eyev = Vec3::new(0.0, 0.0, -1.0);
        let normalv = Vec3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn light_with_eye_offset_45_deg_normal_unchanged() {
        let (m, p) = setup();
        let eyev = Vec3::new(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = Vec3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_45deg() {
        let (m, p) = setup();

        let eyev = Vec3::new(0.0, 0.0, -1.0);
        let normalv = Vec3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let (m, p) = setup();
        let eyev = Vec3::new(0.0, -SQRT_2 / 2.0, -SQRT_2 / 2.0);
        let normalv = Vec3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_eye_behind_surface() {
        let (m, p) = setup();
        let eyev = Vec3::new(0.0, 0.0, -1.0);
        let normalv = Vec3::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = lighting(m, &light, p, eyev, normalv);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
