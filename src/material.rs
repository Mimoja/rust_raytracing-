use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::raytrace::RayCaster;
use glm::*;

pub trait Scatter {
    fn scatter(&self, r: &Ray, hr: HitRecord) ->  (Ray, Vector3<f64>);
}

pub struct Lambertian {
	pub Albedo: Vector3<f64>,
}

impl Scatter for Lambertian{
    fn scatter(&self, r: &Ray, collision: HitRecord) -> (Ray, Vector3<f64>){
		let target = collision.p + collision.normal + RayCaster::random_in_unit_sphere();
		let scattered = Ray::new(collision.p, target - collision.p);
		let attenuation = self.Albedo;
		return (scattered, attenuation);
	}
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64>{
	return v * 2.0 - n * dot(v, n);
}

pub struct Metal {
	pub Albedo: Vector3<f64>,
}


impl Scatter for Metal{
    fn scatter(&self, r: &Ray, collision: HitRecord) -> (Ray, Vector3<f64>){
		let reflect = reflect(r.direction(), collision.normal);
		let scattered = Ray::new(collision.p, reflect);
		let attenuation = self.Albedo;
		return (scattered, attenuation);
	}
}