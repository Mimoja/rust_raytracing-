use crate::ray::Ray;
use crate::material::*;
use glm::*;

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
	pub number_of_steps: u32,
	pub material: Option<Box<dyn Scatter>>,
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vector3<f64> = r.position() - self.center;
        let a: f64 = dot(r.direction(), r.direction());
        let b: f64 = dot(oc, r.direction());
        let c: f64 = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t_to_sphere = (-b - sqrt(discriminant)) / a;
            if t_to_sphere < t_max && t_to_sphere > t_min {
                let p = r.point_at_parameter(t_to_sphere);
                return Some(HitRecord {
                    t: t_to_sphere,
                    p: p,
                    normal: normalize(p - self.center),
					number_of_steps: 1,
					material: Some(Box::new(Metal{
                    Albedo:  Vector3::new(1.0, 0.1, 0.3),
                })),
                });
            }
            let t_to_sphere = (-b + sqrt(discriminant)) / a;
            if t_to_sphere < t_max && t_to_sphere > t_min {
                let p = r.point_at_parameter(t_to_sphere);
                return Some(HitRecord {
                    t: t_to_sphere,
                    p: p,
                    normal: normalize(p - self.center),
					number_of_steps: 1,
					material: Some(Box::new(Metal{
                    Albedo:  Vector3::new(1.0, 0.1, 0.3),
                })),
                });
            }
        }
        return None;
    }
}

pub struct StepSphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}

impl StepSphere {
    pub fn new(center: Vector3<f64>, radius: f64) -> StepSphere {
        StepSphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hit for StepSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

		let mut t = t_min;
		for i in 0..64{
			let dist: f64 = length(r.point_at_parameter(t) - self.center) - self.radius ;

			if dist < 1e-9 {
				// We're inside the surface!
				let p = r.point_at_parameter(t);
				return Some(HitRecord {
					t: t,
					p: p,
					normal: normalize(p - self.center),
					number_of_steps: i + 1,
					material: None,
				});
			}
			// Move along the view ray
			t += dist;

			if t >= t_max {
				// Gone too far; give up
				return None;
			}
		}
		return None;
    }
}
pub struct Plane {
    pub normal: Vector3<f64>,
    pub point_on_plane: Vector3<f64>,
}

impl Hit for Plane {
    fn hit(&self, r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        let denom = dot(r.direction(), self.normal);
        if denom > 0.01 {
            let t = dot(r.position(), self.normal) / denom;
            let p = r.point_at_parameter(t);
            return Some(HitRecord {
                t: t,
                p: p,
                normal: self.normal,
				number_of_steps: 1,
				material: None,
            });
        }
        return None;
    }
}

pub struct Cube {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl Hit for Cube {
    fn hit(&self, r: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        let vmin = (self.min - r.position()) * r.direction();
        let vmax = (self.max - r.position()) * r.direction();

        let mut tmin = vmin.x;
        let mut tmax = vmax.x;

        if tmin > tmax {
            let tmp = tmin;
            tmin = tmax;
            tmax = tmp;
        }

        let mut tymin = vmin.y;
        let mut tymax = vmax.y;

        if tymin > tymax {
            let tmp = tymin;
            tymin = tymax;
            tymax = tmp;
        }

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = vmin.z;
        let mut tzmax = vmax.z;

        if tzmin > tzmax {
            let tmp = tzmin;
            tzmin = tzmax;
            tzmax = tmp;
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

 		let mut t = tmin; 
 
        if t < 0.0 { 
            t = tmax; 
            if t < 0.0 {return None; }
        }

		let hit = r.point_at_parameter(t);
		let	c = (self.min + self.max) * 0.5;
    	let p = hit - c;
    	let d = (self.min - self.max) * 0.5;
    	let bias = 1.000001;

  		let normal = normalize(
			  Vector3::new(p.x / abs(d.x) * bias,
			  p.y / abs(d.y) * bias,
			  p.z / abs(d.z) * bias)
		  );
 
        return Some(HitRecord {
            t: t,
            p: hit,
            normal: normal,
			number_of_steps: 1,
			material: None,
        });
    }
}
