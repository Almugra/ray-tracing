use crate::{
    hit::hitrecord::HitRecord,
    ray::{random_unit_vector, unit_vec, Ray},
    Color, Vector3,
};

use super::Material;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    #[allow(unused)]
    pub fn new(color: Vector3, fuzz: f32) -> Self {
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
        let attenuation = self.albedo;
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        None
    }
}

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - (2.0 * v.dot(n) * n)
}
