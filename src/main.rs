pub mod utility;

use std::io::Write;

use glam::f32::Vec3;
use utility::{hitrecord::HitRecord, hittable::Hittable};

use crate::utility::{camera::Camera, hitlist::HitList, ray::Ray, sphere::Sphere};

fn main() {
    let mut world: HitList<Sphere> = HitList::default();
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));

    let camera = Camera::new(16.0 / 9.0, 400.0, 1.0, 2.0);

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

/// Calculate the color of a ray
pub fn ray_color<T: Hittable>(ray: &Ray, world: &HitList<T>) -> Vec3 {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.0, f32::MAX, &mut hit_record) {
        return (hit_record.normal + Vec3::ONE) * 0.5;
    }

    let unit_direction = unit_vec(ray.direction);
    let t = (unit_direction.y + 1.0) * 0.5;
    Vec3::ONE * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0)
}

pub fn unit_vec(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn write_color(v: Vec3) {
    println!(
        "{} {} {}",
        (255.999 * v.x) as usize,
        (255.999 * v.y) as usize,
        (255.999 * v.z) as usize
    );
}
