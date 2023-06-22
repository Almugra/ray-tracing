use std::sync::Arc;

use crate::{
    hit::hitrecord::HitRecord,
    ray::{random_unit_vector, Ray},
    textures::texture::Texture,
    Color, Vector3,
};

use super::Material;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    #[allow(unused)]
    pub fn new(color: Arc<dyn Texture>) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, scatter_direction, ray_in.time);
        let attenuation = self
            .albedo
            .value(hit_record.uv.0, hit_record.uv.1, &hit_record.point);
        Some((attenuation, scattered))
    }
}

fn near_zero(v: &Vector3) -> bool {
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}
