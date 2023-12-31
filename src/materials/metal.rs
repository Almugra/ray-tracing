use std::sync::Arc;

use crate::{
    hit::hitrecord::HitRecord,
    ray::{random_unit_vector, unit_vec, Ray},
    textures::texture::Texture,
    Color, Vector3,
};

use super::Material;

pub struct Metal {
    pub albedo: Box<dyn Texture>,
    pub fuzz: f32,
}

impl Metal {
    #[allow(unused)]
    pub fn new(color: Box<dyn Texture>, fuzz: f32) -> Self {
        Self {
            albedo: color,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(unit_vec(ray_in.direction), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_unit_vector(),
            ray_in.time,
        );
        let attenuation = self
            .albedo
            .value(hit_record.uv.0, hit_record.uv.1, &hit_record.point);
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        None
    }
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - (2.0 * v.dot(n) * n)
}
