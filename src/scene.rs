use crate::hittable::*;
use crate::ray::Ray;

pub struct Scene {
    pub objects: Vec<Box<dyn Hit>>,
}

impl Hit for Scene {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut best_hit: Option<HitRecord> = None;
        let mut closest: f64 = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, closest) {
                closest = hit.t;
                best_hit = Some(hit);
            }
        }
        return best_hit;
    }
}
