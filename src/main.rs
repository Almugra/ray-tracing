use crate::{camera::Camera, hit::HitList, objects::sphere::Sphere, ray::ray_color};
use glam::f32::Vec3;
use rand::Rng;
use std::io::Write;

mod camera;
mod hit;
mod objects;
mod ray;

fn main() {
    let mut world: HitList<Sphere> = HitList::default();
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    let samples_per_pixel = 100.0;
    let mut rng = rand::thread_rng();

    let camera = Camera::new(16.0 / 9.0, 400.0, 1.0, 2.0);

    println!("P3\n{} {}\n255", camera.image.width, camera.image.height);

    (0..camera.image.height as usize).for_each(|j| {
        let j = camera.image.height as usize - 1 - j;
        eprint!("\rScanlines remaining: {:004}", j);
        std::io::stderr().flush().unwrap();
        (0..camera.image.width as usize).for_each(|i| {
            let mut pixel_color = Vec3::ZERO;
            (0..samples_per_pixel as usize).for_each(|_| {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (camera.image.width - 1.0);
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / (camera.image.height - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            });

            write_color(pixel_color, 100.0);
        });
    });

    eprintln!("\nDone.");
}

pub fn write_color(v: Vec3, samples_per_pixel: f32) {
    let scale = 1.0 / samples_per_pixel;
    let r = v.x * scale;
    let g = v.y * scale;
    let b = v.z * scale;

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as usize,
        (256.0 * g.clamp(0.0, 0.999)) as usize,
        (256.0 * b.clamp(0.0, 0.999)) as usize,
    );
}
