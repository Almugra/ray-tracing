use crate::{
    camera::Camera,
    hit::HitList,
    objects::sphere::Sphere,
    ray::{ray_color, Ray},
};
use glam::f32::Vec3;
use std::io::Write;

mod camera;
mod hit;
mod objects;
mod ray;

fn main() {
    let mut world: HitList<Sphere> = HitList::default();
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));

    let camera = Camera::new(4.0 / 3.0, 400.0, 1.0, 2.0);

    println!("P3\n{} {}\n255", camera.image.width, camera.image.height);

    (0..camera.image.height as usize).for_each(|j| {
        let j = camera.image.height as usize - 1 - j;
        eprint!("\rScanlines remaining: {j}");
        std::io::stderr().flush().unwrap();
        (0..camera.image.width as usize).for_each(|i| {
            let u = i as f32 / (camera.image.width - 1.0);
            let v = j as f32 / (camera.image.height - 1.0);
            let ray = Ray {
                origin: camera.origin,
                direction: camera.viewport.lower_left_corner
                    + camera.viewport.horizontal * u
                    + camera.viewport.vertical * v
                    - camera.origin,
            };
            let pixel_color = ray_color(&ray, &world);

            write_color(pixel_color);
        });
    });
    eprintln!("\nDone.");
}

pub fn write_color(v: Vec3) {
    println!(
        "{} {} {}",
        (255.999 * v.x) as usize,
        (255.999 * v.y) as usize,
        (255.999 * v.z) as usize
    );
}
