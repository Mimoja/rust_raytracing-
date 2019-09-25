use crate::ray::Ray;
use glm::*;

pub struct Camera {
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    origin: Vector3<f64>,
}

impl Camera {
    pub fn new(position: Vector3<f64>, normal: Vector3<f64>, aspect_ration: f64) -> Camera {
        return Camera {
            origin: position,
            lower_left_corner: (Vector3::new(-1.0 * aspect_ration, -1.0, 0.0) + normalize(normal)),
            vertical: Vector3::new(0.0, 2.0 + normal.y, 0.0),
            horizontal: Vector3::new(aspect_ration * (2.0 + normal.x), 0.0, 0.0),
        };
    }

    pub fn get_ray_perspective(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v,
        );
    }

    pub fn get_ray_orthogonal(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.lower_left_corner + Vector3::new(u * self.horizontal.x, v * self.vertical.y, 1.0),
            Vector3::new(0.0, 0.0, -1.0),
        );
    }
}
