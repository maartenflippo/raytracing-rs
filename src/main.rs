mod camera;
mod hittable;
mod material;
mod math;
mod ray;

use std::{io::Write, rc::Rc};

use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use math::Color;
use rand::Rng;
use ray::Ray;

use crate::{
    camera::Camera,
    hittable::Sphere,
    material::{Dielectric, Lambertian, Material, Metal},
    math::Vec3,
};

fn ray_color(rng: &mut impl Rng, ray: Ray, world: &HittableList, depth: usize) -> Color {
    if depth == 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::MAX) {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            return attenuation * ray_color(rng, scattered, world, depth - 1);
        }

        return Color::zero();
    }

    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(w: &mut impl Write, color: &math::Color, samples_per_pixel: usize) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    let r = f64::sqrt(r * scale);
    let g = f64::sqrt(g * scale);
    let b = f64::sqrt(b * scale);

    writeln!(
        w,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u32,
        (256.0 * g.clamp(0.0, 0.999)) as u32,
        (256.0 * b.clamp(0.0, 0.999)) as u32
    )
    .unwrap();
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: usize = 400;
    const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let mut world = HittableList::new();

    let mut rng = rand::thread_rng();
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let material_right: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        Rc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    )));

    let camera = Camera::new();

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    let bar = ProgressBar::new((WIDTH * HEIGHT) as u64);

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut color = Color::zero();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);
                color += ray_color(&mut rng, ray, &world, MAX_DEPTH);
            }

            write_color(&mut std::io::stdout(), &color, SAMPLES_PER_PIXEL);
            bar.inc(1);
        }
    }

    bar.finish();
}
