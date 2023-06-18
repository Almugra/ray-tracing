use crate::{
    hit::HitList,
    materials::{dialectric::Dialectric, lambertian::Lambertian, metal::Metal},
    objects::sphere::Sphere,
    ray::ray_color,
    view::{camera::Camera, image::Image},
};
use glam::f32::Vec3;
use rand::Rng;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::sync::{
    atomic::{AtomicU64, Ordering::Relaxed},
    Arc,
};
use std::{io::Write, time::Instant};

mod hit;
mod materials;
mod objects;
mod ray;
mod view;

type Point3 = Vec3;

fn main() {
    let world = build_world();

    let samples_per_pixel = 100.0;
    let max_depth = 50;

    let ar = 4.0 / 3.0;
    let image = Image::new(ar, 400.0);

    let lookfrom = Point3::new(14.0, 2.8, 2.5);
    let lookat = Point3::new(-8.0, -0.3, -0.6);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.3;
    let dist_to_focus = 10.0;
    let camera = Camera::new(lookfrom, lookat, vup, 18, ar, aperture, dist_to_focus);

    println!("P3\n{} {}\n255", image.width, image.height);

    let now = Instant::now();

    let mut colors = vec![Vec3::default(); (image.height * image.width) as usize];

    let max = colors.len();
    let count = AtomicU64::new(1);
    colors
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel_color)| {
            let j = index / image.width as usize;
            let i = index % image.width as usize;
            let o_j = image.height as usize - 1 - j;

            let mut rng = rand::thread_rng();

            for _ in 0..samples_per_pixel as usize {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (image.width - 1.0);
                let v = (o_j as f32 + rng.gen_range(0.0..1.0)) / (image.height - 1.0);
                let ray = camera.get_ray(u, v);
                *pixel_color += ray_color(&ray, &world, max_depth);
            }

            eprint!("\rpixels: {:004}/{}", count.fetch_add(1, Relaxed), max);
            std::io::stderr().flush().unwrap();
        });

    for x in colors {
        write_color(x, samples_per_pixel);
    }

    eprintln!("Done.\n{:?}", now.elapsed());
}

fn build_world() -> HitList<Sphere> {
    let mut world: HitList<Sphere> = HitList::default();

    let mat_ground = Lambertian::new(Vec3::new(0.1, 0.2, 0.2));
    world.push(Sphere::new(
        Point3::new(0.0, -10000.0, 0.0),
        10000.0,
        Arc::new(mat_ground),
    ));

    let mat_center = Metal::new(Vec3::new(0.9, 0.5, 0.1), 0.9);
    world.push(Sphere::new(
        Point3::new(1.0, 1.0, -1.0),
        1.0,
        Arc::new(mat_center),
    ));

    let mat_left = Dialectric::new(1.5);
    world.push(Sphere::new(
        Point3::new(4.0, 1.0, 2.4),
        1.0,
        Arc::new(mat_left),
    ));

    let mat_right = Metal::new(Vec3::new(0.2, 0.1, 0.8), 0.1);
    world.push(Sphere::new(
        Point3::new(2.5, 1.0, 1.3),
        1.0,
        Arc::new(mat_right),
    ));
    world
}

pub fn write_color(v: Vec3, samples_per_pixel: f32) {
    let scale = 1.0 / samples_per_pixel;
    let r = (v.x * scale).sqrt();
    let g = (v.y * scale).sqrt();
    let b = (v.z * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as usize,
        (256.0 * g.clamp(0.0, 0.999)) as usize,
        (256.0 * b.clamp(0.0, 0.999)) as usize,
    );
}
