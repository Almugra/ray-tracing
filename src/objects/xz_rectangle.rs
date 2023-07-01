use glam::Vec3;

use crate::{hit::hittable::Hittable, materials::Material};
use std::sync::Arc;

pub struct XZRectangle {
    pub material: Option<Arc<dyn Material>>,
    pub x: (f32, f32),
    pub z: (f32, f32),
    pub k: f32,
}

impl XZRectangle {
    #[allow(unused)]
    pub fn new(x: (f32, f32), z: (f32, f32), k: f32, material: Arc<dyn Material>) -> Self {
        Self {
            material: Some(material),
            x,
            z,
            k,
        }
    }
}

unsafe impl Send for XZRectangle {}
unsafe impl Sync for XZRectangle {}

impl Hittable for XZRectangle {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut crate::hit::hitrecord::HitRecord,
    ) -> bool {
        let t = (self.k - ray.origin.y) / (ray.direction.y);
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return false;
        }
        hit_record.uv.0 = (x - self.x.0) / (self.x.1 - self.x.0);
        hit_record.uv.1 = (z - self.z.0) / (self.z.1 - self.z.0);
        hit_record.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);
        true
    }
}
