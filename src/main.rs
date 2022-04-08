use indicatif::ProgressBar;

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

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
            bar.inc(1);
        }
    }

    bar.finish();
}
