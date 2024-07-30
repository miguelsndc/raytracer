use super::{sphere::Sphere, transforms::Transform};
use crate::primitives::{point::Point, tuple::Tuple, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        return self.origin + self.direction * t;
    }

    pub fn origin(&self) -> Point {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }
}

impl Transform for Ray {
    fn transform(self, transform: &crate::primitives::matrix4f::Matrix4f) -> Self {
        return Ray {
            origin: (*transform) * self.origin,
            direction: (*transform) * self.direction,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_initialization() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin(), origin);
        assert_eq!(r.direction(), direction);
    }

    #[test]
    fn ray_position_at_t() {
        let origin = Point::new(2.0, 3.0, 4.0);
        let direction = Vec3::new(1.0, 0.0, 0.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
    }

    #[test]
    fn ray_transformable() {
        {
            let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0));
            let transformed_ray = r.translate(3.0, 4.0, 5.0).transform();

            assert_eq!(transformed_ray.origin(), Point::new(4.0, 6.0, 8.0));
            assert_eq!(transformed_ray.direction(), Vec3::new(0.0, 1.0, 0.0));
        }
        {
            let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0));
            let transformed_ray = r.scale(2.0, 3.0, 4.0).transform();

            assert_eq!(transformed_ray.origin(), Point::new(2.0, 6.0, 12.0));
            assert_eq!(transformed_ray.direction(), Vec3::new(0.0, 3.0, 0.0));
        }
    }
}
