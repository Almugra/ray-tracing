use rand::Rng;

use crate::{
    ray::{unit_vec, Ray},
    Point3, Vector3,
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f32,
    time: (f32, f32),
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vector3,
        vfov: usize,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time: (f32, f32),
    ) -> Self {
        let theta = (vfov as f32).to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_widt = aspect_ratio * viewport_height;

        let w = unit_vec(lookfrom - lookat);
        let u = unit_vec(vup.cross(w));
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_widt * u;
        let vertical = focus_dist * viewport_height * v;

        Self {
            origin: lookfrom,
            lower_left_corner: lookfrom - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
            time: (time.0, time.1),
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let mut rng = rand::thread_rng();

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            time: rng.gen_range(self.time.0..self.time.1),
        }
    }
}

fn random_in_unit_disk() -> Vector3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
