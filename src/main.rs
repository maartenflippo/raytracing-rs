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
    math::{Point3, Vec3},
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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const WIDTH: usize = 800;
    const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_DEPTH: usize = 50;

    let mut rng = rand::thread_rng();
    // let mut world = HittableList::new();

    // let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    // let material_right: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // world.add(Box::new(Sphere::new(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     Rc::clone(&material_ground),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     Rc::clone(&material_center),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     Rc::clone(&material_left),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     Rc::clone(&material_left),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     Rc::clone(&material_right),
    // )));
    let world = random_scene(&mut rng);

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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

                let ray = camera.get_ray(&mut rng, u, v);
                color += ray_color(&mut rng, ray, &world, MAX_DEPTH);
            }

            write_color(&mut std::io::stdout(), &color, SAMPLES_PER_PIXEL);
            bar.inc(1);
        }
    }

    bar.finish();
}

fn random_scene(rng: &mut impl Rng) -> HittableList {
    let mut world = HittableList::new();

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(rng) * Color::random(rng);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}
