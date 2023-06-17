use glam::Vec3;

use crate::{
    hit::hitrecord::HitRecord,
    ray::{random_unit_vector, Ray},
};

use super::Material;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

fn near_zero(v: &Vec3) -> bool {
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}
