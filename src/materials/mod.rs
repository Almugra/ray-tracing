#![allow(unused)]
use crate::{hit::hitrecord::HitRecord, ray::Ray, Color, Point3};

pub mod dialectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
