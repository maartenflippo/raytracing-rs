mod camera;
mod hittable;
mod math;
mod ray;

use std::io::Write;

use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use math::Color;
use rand::Rng;
use ray::Ray;

use crate::{camera::Camera, hittable::Sphere, math::Vec3};

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::MAX) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(w: &mut impl Write, color: &math::Color, samples_per_pixel: usize) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    let r = r * scale;
    let g = g * scale;
    let b = b * scale;

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

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    let bar = ProgressBar::new((WIDTH * HEIGHT) as u64);

    let mut rng = rand::thread_rng();

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut color = Color::zero();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world);
            }

            write_color(&mut std::io::stdout(), &color, SAMPLES_PER_PIXEL);
            bar.inc(1);
        }
    }

    bar.finish();
}
