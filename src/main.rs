#![allow(dead_code)]

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    let mut j = IMAGE_HEIGHT - 1;
    while j > 0 {
        (0..IMAGE_WIDTH).for_each(|i| {
            let r = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let g = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as usize;
            let ig = (255.999 * g) as usize;
            let ib = (255.999 * b) as usize;

            println!("{ir} {ig} {ib}\n")
        });
        j -= 1;
    }
}
