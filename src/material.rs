use std::cell::RefCell;

use crate::{
    hittable::HitRecord,
    math::{Color, Vec3},
    ray::Ray,
};

pub trait Material {
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
    rng: RefCell<rand::rngs::ThreadRng>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo,
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction =
            rec.normal + Vec3::random_unit_vector(&mut *self.rng.borrow_mut());

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
    rng: RefCell<rand::rngs::ThreadRng>,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().unit().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(&mut *self.rng.borrow_mut()),
        );

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
