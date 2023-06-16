#![allow(dead_code)]

pub mod utility;

use std::io::Write;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    (0..IMAGE_HEIGHT).for_each(|j| {
        let j = IMAGE_HEIGHT - 1 - j;
        eprint!("\rScanlines remaining: {j} ");
        std::io::stderr().flush().unwrap();
        (0..IMAGE_WIDTH).for_each(|i| {
            let pixel_color = utility::vec3::Vec3 {
                x: (i as f64) / (IMAGE_WIDTH - 1) as f64,
                y: (j as f64) / (IMAGE_HEIGHT - 1) as f64,
                z: 0.25,
            };

            pixel_color.write_color();
        });
    });
    eprintln!("\nDone.");
}
