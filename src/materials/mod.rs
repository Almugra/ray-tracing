use crate::{hit::hitrecord::HitRecord, ray::Ray, Color};

pub mod dialectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}
