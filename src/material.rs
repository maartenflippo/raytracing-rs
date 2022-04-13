use rand::Rng;
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

pub struct Dielectric {
    ir: f64,
    rng: RefCell<rand::rngs::ThreadRng>,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric {
            ir,
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = f64::min((-unit_direction).dot(rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let random_reflect =
            reflectance(cos_theta, refraction_ratio) > self.rng.borrow_mut().gen_range(0.0..1.0);
        let direction = if cannot_refract || random_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}
