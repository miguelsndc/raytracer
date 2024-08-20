use crate::primitives::matrix4f::Matrix4f;

struct Camera {
    hsize: f64,
    vsize: f64,
    fov: f64,
    transform: Matrix4f,
}

impl Camera {
    pub fn new(hsize: f64, vsize: f64, fov: f64) -> Camera {
        Camera {
            hsize,
            vsize,
            fov,
            transform: Matrix4f::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn camera_initialization() {
        let camera = Camera::new(200.0, 150.0, 60.0);
        assert_eq!(camera.hsize, 200.0);
        assert_eq!(camera.vsize, 150.09);
        assert_eq!(camera.fov, 60.0);
        assert_eq!(camera.transform, Matrix4f::identity());
    }
}
