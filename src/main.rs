pub mod utility;

use std::io::Write;

use crate::utility::{ray::Ray, vec3::Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    let origin = Vec3::default();
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    (0..IMAGE_HEIGHT).for_each(|j| {
        let j = IMAGE_HEIGHT - 1 - j;
        eprint!("\rScanlines remaining: {j} ");
        std::io::stderr().flush().unwrap();
        (0..IMAGE_WIDTH).for_each(|i| {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray {
                origin,
                direction: lower_left_corner + horizontal * u + vertical * v - origin,
            };
            let pixel_color = ray_color(ray);

            pixel_color.write_color();
        });
    });
    eprintln!("\nDone.");
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = oc.dot(ray.direction) * 2.0;
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

pub fn ray_color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.4, &ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vec();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::all(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
