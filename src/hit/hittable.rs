use crate::ray::Ray;

use super::hitrecord::HitRecord;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}
