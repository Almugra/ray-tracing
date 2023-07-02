use crate::{
    objects::sphere::Sphere,
    ray::ray_color,
    view::{camera::Camera, image::Image},
};
use glam::Vec3;
use hit::hitlist::HitList;
use materials::{dialectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian};
use objects::{
    block::Block, xy_rectangle::XYRectangle, xz_rectangle::XZRectangle, yz_rectangle::YZRectangle,
};
use rand::Rng;
use ray::{rotate::RotateY, translate::Translate};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::io::Write;
use std::time::Instant;
use std::{
    io::StdoutLock,
    sync::{
        atomic::{AtomicU64, Ordering::Relaxed},
        Arc,
    },
};
use textures::solid::SolidColor;

type Vector3 = Vec3;
type Point3 = Vec3;
type Color = Vec3;

mod hit;
mod materials;
mod objects;
mod ray;
mod textures;
mod view;

fn main() {
    let world = cornell_box();

    let samples_per_pixel = 100.0;
    let max_depth = 50;

    let ar = 1.0;
    let image = Image::new(ar, 400.0);

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = (lookfrom - lookat).length();
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40,
        ar,
        aperture,
        dist_to_focus,
        (0.0, 1.0),
    );

    println!("P3\n{} {}\n255", image.width, image.height);

    let now = Instant::now();

    let mut colors = vec![Vector3::default(); (image.height * image.width) as usize];

    let background = Color::ZERO;
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
                *pixel_color += ray_color(&ray, background, &world, max_depth);
            }

            eprint!("\rpixels: {:004}/{}", count.fetch_add(1, Relaxed), max);
            std::io::stderr().flush().unwrap();
        });

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for x in colors {
        write_color(&mut lock, x, samples_per_pixel);
    }
    drop(lock);

    eprintln!("\n{:?}", now.elapsed());
}

fn cornell_box() -> HitList {
    let mut objects = HitList::default();

    let red = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    objects.push(Arc::new(YZRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        green,
    )));
    objects.push(Arc::new(YZRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        red,
    )));
    objects.push(Arc::new(XZRectangle::new(
        (113.0, 443.0),
        (127.0, 432.0),
        554.0,
        light,
    )));
    objects.push(Arc::new(XZRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));
    objects.push(Arc::new(XZRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    )));
    objects.push(Arc::new(XYRectangle::new(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    )));

    let block = Arc::new(Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let block = Arc::new(RotateY::new(block, 15.0));
    let block = Arc::new(Translate::new(Point3::new(265.0, 0.0, 295.0), block));
    objects.push(block);

    let block1 = Arc::new(Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    let block1 = Arc::new(RotateY::new(block1, -18.0));
    let block1 = Arc::new(Translate::new(Point3::new(130.0, 0.0, 65.0), block1));
    objects.push(block1);

    objects
}

pub fn write_color(lock: &mut StdoutLock, v: Vector3, samples_per_pixel: f32) {
    let scale = 1.0 / samples_per_pixel;
    let r = (v.x * scale).sqrt();
    let g = (v.y * scale).sqrt();
    let b = (v.z * scale).sqrt();

    writeln!(
        lock,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as usize,
        (256.0 * g.clamp(0.0, 0.999)) as usize,
        (256.0 * b.clamp(0.0, 0.999)) as usize,
    )
    .unwrap();
}
