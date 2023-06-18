use crate::{
    hit::HitList,
    materials::metal::Metal,
    objects::sphere::Sphere,
    ray::ray_color,
    view::{camera::Camera, image::Image},
};
use glam::f32::Vec3;
use objects::moving_sphere::MovingSphere;
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

    let samples_per_pixel = 400.0;
    let max_depth = 50;

    let ar = 4.0 / 3.0;
    let image = Image::new(ar, 400.0);

    let lookfrom = Point3::new(0.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.7, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = (lookfrom - lookat).length();
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        50,
        ar,
        aperture,
        dist_to_focus,
        (0.0, 1.0),
    );

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
                let u = (i as f32 + rng.gen::<f32>()) / (image.width - 1.0);
                let v = (o_j as f32 + rng.gen::<f32>()) / (image.height - 1.0);
                let ray = camera.get_ray(u, v);
                *pixel_color += ray_color(&ray, &world, max_depth);
            }

            eprint!("\rpixels: {:004}/{}", count.fetch_add(1, Relaxed), max);
            std::io::stderr().flush().unwrap();
        });

    for x in colors {
        write_color(x, samples_per_pixel);
    }

    eprintln!("\n{:?}", now.elapsed());
}

fn build_world() -> HitList {
    let mut world: HitList = HitList::default();

    let mat_ground = Metal::new(Vec3::new(0.2, 0.2, 0.2), 0.8);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(mat_ground),
    )));

    let mat_right = Metal::new(Vec3::new(0.2, 0.7, 0.9), 0.3);
    world.push(Box::new(MovingSphere::new(
        (Point3::new(1.0, 1.0, -1.0), Point3::new(1.0, 1.3, -1.0)),
        (0.0, 1.0),
        1.0,
        Arc::new(mat_right),
    )));

    let mat_left = Metal::new(Vec3::new(0.9, 0.2, 0.1), 0.6);
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 1.0, -1.0),
        1.0,
        Arc::new(mat_left),
    )));
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
