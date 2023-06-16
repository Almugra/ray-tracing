pub mod utility;

use std::io::Write;

use utility::{hitrecord::HitRecord, hittable::Hittable};

use crate::utility::{hitlist::HitList, ray::Ray, sphere::Sphere, vec3::Vec3};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

// Camera parameters
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    let mut world: HitList<Sphere> = HitList::default();
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));

    let camera_origin = Vec3::default();
    let viewport_horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let viewport_lower_left_corner = camera_origin
        - viewport_horizontal / 2.0
        - viewport_vertical / 2.0
        - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    (0..IMAGE_HEIGHT).for_each(|j| {
        let j = IMAGE_HEIGHT - 1 - j;
        eprint!("\rScanlines remaining: {j}");
        std::io::stderr().flush().unwrap();
        (0..IMAGE_WIDTH).for_each(|i| {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray {
                origin: camera_origin,
                direction: viewport_lower_left_corner
                    + viewport_horizontal * u
                    + viewport_vertical * v
                    - camera_origin,
            };
            let pixel_color = ray_color(&ray, &world);

            pixel_color.write_color();
        });
    });
    eprintln!("\nDone.");
}

/// Calculate the color of a ray
pub fn ray_color<T: Hittable>(ray: &Ray, world: &HitList<T>) -> Vec3 {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.0, f64::MAX, &mut hit_record) {
        return (hit_record.normal + Vec3::all(1.0)) * 0.5;
    }

    let unit_direction = ray.direction.unit_vec();
    let t = (unit_direction.y + 1.0) * 0.5;
    Vec3::all(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0)
}
