use glm::*;

pub struct Ray {
    position: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(position: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            position: position,
            direction: normalize(direction),
        }
    }

    pub fn position(&self) -> Vector3<f64> {
        return self.position;
    }
    pub fn direction(&self) -> Vector3<f64> {
        return self.direction;
    }
    pub fn point_at_parameter(&self, t: f64) -> Vector3<f64> {
        return self.position + self.direction * t;
    }
}
