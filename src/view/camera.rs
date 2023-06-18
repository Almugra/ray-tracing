use glam::Vec3;

use crate::{
    ray::{unit_vec, Ray},
    Point3,
};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: usize,
        aspect_ratio: f32,
    ) -> Self {
        let theta = (vfov as f32).to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_widt = aspect_ratio * viewport_height;

        let w = unit_vec(lookfrom - lookat);
        let u = unit_vec(vup.cross(w));
        let v = w.cross(u);

        let horizontal = viewport_widt * u;
        let vertical = viewport_height * v;

        Self {
            origin: lookfrom,
            lower_left_corner: lookfrom - horizontal / 2.0 - vertical / 2.0 - w,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin,
        }
    }
}
