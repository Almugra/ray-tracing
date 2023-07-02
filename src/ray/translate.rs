use super::Ray;
use crate::{hit::hittable::Hittable, Vector3};
use std::sync::Arc;

pub struct Translate {
    offset: Vector3,
    object: Arc<dyn Hittable>,
}

impl Translate {
    #[allow(unused)]
    pub fn new(offset: Vector3, object: Arc<dyn Hittable>) -> Self {
        Self { offset, object }
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        ray: &super::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut crate::hit::hitrecord::HitRecord,
    ) -> bool {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if !self.object.hit(&moved_ray, t_min, t_max, hit_record) {
            return false;
        }

        hit_record.point += self.offset;
        hit_record.set_face_normal(&moved_ray, hit_record.normal);

        true
    }
}
