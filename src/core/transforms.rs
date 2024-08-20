use crate::primitives::{
    matrix4f::{Matrix4f, MATRIX_SIZE},
    tuple::Tuple,
    vec3::Vec3,
};

pub trait Transform {
    fn transform(self, transform: &Matrix4f) -> Self;

    fn translate(self, x: f64, y: f64, z: f64) -> TransformChainer<Self>
    where
        Self: Sized,
    {
        return TransformChainer {
            m: Transformations::translate(x, y, z),
            x: self,
        };
    }

    fn scale(self, x: f64, y: f64, z: f64) -> TransformChainer<Self>
    where
        Self: Sized,
    {
        return TransformChainer {
            m: Transformations::scale(x, y, z),
            x: self,
        };
    }

    fn rotate_y(self, radians: f64) -> TransformChainer<Self>
    where
        Self: Sized,
    {
        return TransformChainer {
            m: Transformations::rotate_y(radians),
            x: self,
        };
    }

    fn rotate_x(self, radians: f64) -> TransformChainer<Self>
    where
        Self: Sized,
    {
        return TransformChainer {
            m: Transformations::rotate_x(radians),
            x: self,
        };
    }

    fn rotate_z(self, radians: f64) -> TransformChainer<Self>
    where
        Self: Sized,
    {
        return TransformChainer {
            m: Transformations::rotate_z(radians),
            x: self,
        };
    }

    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> TransformChainer<Self>
    where
        Self: Sized,
    {
        return TransformChainer {
            m: Transformations::shear(xy, xz, yx, yz, zx, zy),
            x: self,
        };
    }
}
pub struct Transformations;
impl Transformations {
    pub fn translate(tx: f64, ty: f64, tz: f64) -> Matrix4f {
        let mut trans = Matrix4f::identity();
        trans[(0, MATRIX_SIZE - 1)] = tx;
        trans[(1, MATRIX_SIZE - 1)] = ty;
        trans[(2, MATRIX_SIZE - 1)] = tz;
        return trans;
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Matrix4f {
        let mut sc = Matrix4f::identity();
        sc[(0, 0)] = sx;
        sc[(1, 1)] = sy;
        sc[(2, 2)] = sz;
        return sc;
    }

    pub fn rotate_y(radians: f64) -> Matrix4f {
        let mut rotation = Matrix4f::identity();

        let cos = f64::cos(radians);
        let sin = f64::sin(radians);

        rotation[(0, 0)] = cos;
        rotation[(0, 2)] = sin;
        rotation[(2, 0)] = -sin;
        rotation[(2, 2)] = cos;

        return rotation;
    }

    pub fn rotate_x(radians: f64) -> Matrix4f {
        let mut rotation = Matrix4f::identity();

        let cos = f64::cos(radians);
        let sin = f64::sin(radians);

        rotation[(1, 1)] = cos;
        rotation[(2, 2)] = cos;
        rotation[(2, 1)] = sin;
        rotation[(1, 2)] = -sin;

        return rotation;
    }

    pub fn rotate_z(radians: f64) -> Matrix4f {
        let mut rotation = Matrix4f::identity();

        let cos = f64::cos(radians);
        let sin = f64::sin(radians);

        rotation[(0, 0)] = cos;
        rotation[(1, 1)] = cos;
        rotation[(0, 1)] = -sin;
        rotation[(1, 0)] = sin;

        return rotation;
    }

    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4f {
        let mut sh = Matrix4f::identity();
        sh[(0, 1)] = xy;
        sh[(0, 2)] = xz;

        sh[(1, 0)] = yx;
        sh[(1, 2)] = yz;

        sh[(2, 0)] = zx;
        sh[(2, 1)] = zy;

        return sh;
    }

    pub fn view_transform(from: Vec3, to: Vec3, up: Vec3) -> Matrix4f {
        let forward = (to - from).normalize();
        let left = forward * (up.normalize());
        let true_up = left * forward;

        let orientation = Matrix4f::new_from([
            [left.x(), left.y(), left.z(), 0.0],
            [true_up.x(), true_up.y(), true_up.z(), 0.0],
            [-forward.x(), -forward.y(), -forward.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        return orientation * Transformations::translate(-from.x(), -from.y(), -from.z());
    }
}

pub struct TransformChainer<T> {
    pub m: Matrix4f,
    pub x: T,
}

impl<T> TransformChainer<T>
where
    T: Transform,
{
    pub fn transform(self) -> T {
        return self.x.transform(&self.m);
    }

    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.m = Transformations::translate(x, y, z) * self.m;
        return self;
    }

    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.m = Transformations::scale(x, y, z) * self.m;
        return self;
    }

    pub fn rotate_y(mut self, radians: f64) -> Self {
        self.m = Transformations::rotate_y(radians) * self.m;
        return self;
    }

    pub fn rotate_x(mut self, radians: f64) -> Self {
        self.m = Transformations::rotate_x(radians) * self.m;
        return self;
    }

    pub fn rotate_z(mut self, radians: f64) -> Self {
        self.m = Transformations::rotate_z(radians) * self.m;
        return self;
    }

    pub fn shear(mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        self.m = Transformations::shear(xy, xz, yx, yz, zx, zy) * self.m;
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{point::Point, tuple::Tuple, vec3::Vec3};
    use core::f64;

    #[test]
    fn translates() {
        let t = Transformations::translate(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(t * p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn inverse_of_translate_goes_in_opposite_direction() {
        let t = Transformations::translate(5.0, -3.0, 2.0);
        let inv = t.invert().unwrap();
        let p = Point::new(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn vector_preserved_by_translation() {
        let t = Transformations::translate(5.0, -3.0, 2.0);
        let p = Vec3::new(-3.0, 4.0, 5.0);
        assert_eq!(t * p, p);
    }

    #[test]
    // scaling points makes no sense
    fn scales_vector() {
        let t = Transformations::scale(2.0, 3.0, 4.0);
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(t * p, Vec3::new(2.0, 6.0, 12.0));
    }

    #[test]
    fn inverse_of_scale_shrinks_vector() {
        let t = Transformations::scale(2.0, 3.0, 4.0);
        let inv = t.invert().unwrap();
        let p = Vec3::new(4.0, 6.0, 8.0);
        assert_eq!(inv * p, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let t = Transformations::scale(-1.0, 1.0, 1.0);
        let p = Vec3::new(4.0, 6.0, 8.0);
        assert_eq!(t * p, Vec3::new(-4.0, 6.0, 8.0));
    }

    #[test]
    fn rotation_around_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_q = Transformations::rotate_x(f64::consts::FRAC_PI_4);
        let full_q = Transformations::rotate_x(f64::consts::FRAC_PI_2);

        assert_eq!(
            half_q * p,
            Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_q * p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn shear() {
        let t = Transformations::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let v = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(t * v, Vec3::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn composing_transformations() {
        let p = Point::new(1.0, 0.0, 1.0)
            .rotate_x(f64::consts::FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0)
            .transform();

        assert_eq!(p, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn view_transformation_for_default_orientation() {
        let t = Transformations::view_transform(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert_eq!(t, Matrix4f::identity());
    }

    #[test]
    fn view_transformation_for_positive_z_axis() {
        let t = Transformations::view_transform(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert_eq!(t, Transformations::scale(-1.0, 1.0, -1.0));
    }
    #[test]
    fn view_transform_moves_world() {
        let t = Transformations::view_transform(
            Vec3::new(0.0, 0.0, 8.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert_eq!(t, Transformations::translate(0.0, 0.0, -8.0));
    }
}
