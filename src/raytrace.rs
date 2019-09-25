use crate::camera::Camera;
use crate::hittable::*;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::material::*;

use glm::*;
use rand::*;

pub struct RayCaster {
    pub Image: Vec<Box<Vector3<f64>>>,
    pub size_x: usize,
    pub size_y: usize,
    pub orthonogal: bool,
    pub number_of_renders: u64,
}

impl RayCaster {
    pub fn new(size_x: usize, size_y: usize) -> RayCaster {
        return RayCaster {
            Image: Vec::new(),
            size_x: size_x,
            size_y: size_y,
            orthonogal: false,
            number_of_renders: 0,
        };
    }

    pub fn clearImage(&mut self) {
        self.Image.clear();
    }

    pub fn random_in_unit_sphere() -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vector3::new(
                rng.gen_range(0.0, 0.99),
                rng.gen_range(0.0, 0.99),
                rng.gen_range(0.0, 0.99),
            ) * 2.0
                - 1.0;
            if length(p) < 1.0 {
                return p;
            }
        }
    }

    fn scene(&self) -> Scene {
        let mut scene = Scene {
            objects: Vec::new(),
        };

        let plane = Plane {
            point_on_plane: Vector3::new(0.0, 0.0, -1.0),
            normal: Vector3::new(0.0, -1.0, 0.0),
        };
        //scene.objects.push(Box::new(plane));

        let sphere_origin: Vector3<f64> = Vector3::new(0.5, 0.0, -1.0);
        let sphere = Sphere::new(sphere_origin, 0.5);
        scene.objects.push(Box::new(sphere));

        let sphere_origin: Vector3<f64> = Vector3::new(-0.5, 0.0, -1.0);
        let sphere = StepSphere::new(sphere_origin, 0.5);
        scene.objects.push(Box::new(sphere));

        let sphere_origin: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);
        let sphere = Sphere::new(sphere_origin, 0.5);
        //scene.objects.push(Box::new(sphere));

        let sphere_origin: Vector3<f64> = Vector3::new(0.0, -100.5, -1.0);
        let sphere = Sphere::new(sphere_origin, 100.0);
        scene.objects.push(Box::new(sphere));

        let cube = Cube {
            min: Vector3::new(0.4, 0.0, -0.3),
            max: Vector3::new(0.6, 0.4, -0.5),
        };
        //scene.objects.push(Box::new(cube));

        return scene;
    }

    fn color(&self, r: &Ray, bounces_left: u32) -> Vector3<f64> {
        if bounces_left > 0 {
            if let Some(collision) = self.scene().hit(r, 1e-6, 100.0) {
                let (new_ray, attenuation) = Lambertian{
                    Albedo:  Vector3::new(0.7, 0.6, 0.3),
                }.scatter(r, collision);
                return attenuation * self.color(&new_ray, 
                   bounces_left - 1,
                );
            }
        }

        let norm_r_dir = normalize(r.direction());
        let t = 0.5 * norm_r_dir.y + 1.0;
        return Vector3::new(1.0 as f64, 1.0 as f64, 1.0 as f64) * (1.0 - t)
            + Vector3::new(0.8, 0.6, 1.0) * t;
    }

    pub fn trace(&mut self, time: f64){
                let cam = Camera::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            self.size_x as f64 / self.size_y as f64,
        );
        self.number_of_renders += 1;

        let mut rng = rand::thread_rng();

        for y in 0..self.size_y {
            for x in 0..self.size_x {

                let u = (x as f64 + rng.gen_range(-0.99, 0.99)) / self.size_x as f64;
                let v = (self.size_y as f64 - (y as f64 + rng.gen_range(-0.99, 0.99)))
                / self.size_y as f64;

                //let u = (x as f64 + 0.5 - s as f64 / ns as f64) / size_x as f64;
                //let v = (size_y as f64 -(y as f64 + 0.5 - s as f64 / ns as f64)) / size_y as f64;
                //let u = x as f64 / size_x as f64;
                //let v = (size_y - y) as f64 / size_y as f64;
                
                let perspect: Ray;
                if self.orthonogal {
                    perspect = cam.get_ray_orthogonal(u, v);
                } else {
                    perspect = cam.get_ray_perspective(u, v);
                }

                let c = sqrt(self.color(&perspect, 10));

                let offset = y * self.size_x + x;
                if self.Image.len() <= offset {
                    self.Image.push(Box::new(c));
                } else {
                    self.Image[offset] = Box::new(*self.Image[offset] + c);
                }
            }
        }
    }
    pub fn fill_buffer(&mut self, buffer: &mut [u8]) {

        for (i,_) in self.Image.iter().enumerate() {
            let d = *self.Image[i] / self.number_of_renders as f64;

            buffer[i * 3] = (d.x * 255.99) as u8;
            buffer[i * 3 + 1] = (d.y * 255.9) as u8;
            buffer[i * 3 + 2] = (d.z * 255.9) as u8;
        }
    }
}
