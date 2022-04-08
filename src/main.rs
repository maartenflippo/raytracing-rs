mod math;

use std::io::Write;

use indicatif::ProgressBar;

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
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    let bar = ProgressBar::new((WIDTH * HEIGHT) as u64);

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let r = (i as f64) / (WIDTH - 1) as f64;
            let g = (j as f64) / (HEIGHT - 1) as f64;
            let b = 0.25;

            write_color(&mut std::io::stdout(), &math::Color::new(r, g, b));
            bar.inc(1);
        }
    }

    bar.finish();
}
