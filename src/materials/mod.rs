use glam::Vec3;

use crate::{hit::hitrecord::HitRecord, ray::Ray};

pub mod lambertian;
pub mod metal;
pub mod dialectric;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}
