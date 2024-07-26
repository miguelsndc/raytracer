use super::sphere::Sphere;
use crate::primitives::{point::Point, tuple::Tuple, vec3::Vec3};

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

pub fn intersect(s: Sphere, r: Ray) -> Option<(f64, f64)> {
    let a = r.direction() ^ r.direction();
    let oc = s.center() - r.origin();
    let b = -2.0 * (r.direction ^ (oc));
    let c = (oc ^ oc) - s.radius() * s.radius();

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let t1 = (-b - f64::sqrt(discriminant)) / 2.0 * a;
    let t2 = (-b + f64::sqrt(discriminant)) / 2.0 * a;

    return Some((t1, t2));
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
    fn ray_sphere_intersection() {
        {
            let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Sphere::default();

            let xs = intersect(s, r).unwrap();

            assert_eq!(xs.0, 4.0);
            assert_eq!(xs.1, 6.0);
        }
        {
            let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Sphere::default();

            let xs = intersect(s, r).unwrap();

            assert_eq!(xs.0, 5.0);
            assert_eq!(xs.1, 5.0);
        }
        {
            let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Sphere::default();

            let xs = intersect(s, r).unzip();

            assert_eq!(xs.0, None);
            assert_eq!(xs.1, None);
        }
        {
            let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Sphere::default();

            let xs = intersect(s, r).unwrap();

            assert_eq!(xs.0, -1.0);
            assert_eq!(xs.1, 1.0);
        }
        {
            let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0));
            let s = Sphere::default();

            let xs = intersect(s, r).unwrap();

            assert_eq!(xs.0, -6.0);
            assert_eq!(xs.1, -4.0);
        }
    }
}
