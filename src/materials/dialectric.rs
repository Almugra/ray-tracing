use glam::Vec3;
use rand::Rng;

use crate::ray::{unit_vec, Ray};

use super::{metal::reflect, Material};

pub struct Dialectric {
    pub index_of_refraction: f32,
}

impl Dialectric {
    pub fn new(ir: f32) -> Self {
        Self {
            index_of_refraction: ir,
        }
    }

    fn reflectance(cousine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cousine).powf(5.0)
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        hit_record: &crate::hit::hitrecord::HitRecord,
    ) -> Option<(glam::Vec3, crate::ray::Ray)> {
        let attenuation = Vec3::ONE;
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = unit_vec(ray_in.direction);

        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>()
        {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction);

        Some((attenuation, scattered))
    }
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
    r_out_perp + r_out_parallel
}
