mod hittable;
mod math;
mod ray;

use std::io::Write;

use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use math::Color;
use ray::Ray;

use crate::{
    hittable::Sphere,
    math::{Point3, Vec3},
};

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::MAX) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn write_color(w: &mut impl Write, color: &math::Color) {
    writeln!(
        w,
        "{} {} {}",
        (255.999 * color.x()) as u32,
        (255.999 * color.y()) as u32,
        (255.999 * color.z()) as u32
    )
    .unwrap();
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: usize = 400;
    const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    let bar = ProgressBar::new((WIDTH * HEIGHT) as u64);

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u = (i as f64) / (WIDTH - 1) as f64;
            let v = (j as f64) / (HEIGHT - 1) as f64;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let color = ray_color(&ray, &world);

            write_color(&mut std::io::stdout(), &color);
            bar.inc(1);
        }
    }

    bar.finish();
}
