use glam::Vec3;

use crate::{hit::hittable::Hittable, materials::Material};
use std::sync::Arc;

pub struct YZRectangle {
    pub material: Option<Arc<dyn Material>>,
    pub y: (f32, f32),
    pub z: (f32, f32),
    pub k: f32,
}

impl YZRectangle {
    #[allow(unused)]
    pub fn new(y: (f32, f32), z: (f32, f32), k: f32, material: Arc<dyn Material>) -> Self {
        Self {
            material: Some(material),
            y,
            z,
            k,
        }
    }
}

unsafe impl Send for YZRectangle {}
unsafe impl Sync for YZRectangle {}

impl Hittable for YZRectangle {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut crate::hit::hitrecord::HitRecord,
    ) -> bool {
        let t = (self.k - ray.origin.x) / (ray.direction.x);
        if t < t_min || t > t_max {
            return false;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y.0 || y > self.y.1 || z < self.z.0 || z > self.z.1 {
            return false;
        }
        hit_record.uv.0 = (y - self.y.0) / (self.y.1 - self.y.0);
        hit_record.uv.1 = (z - self.z.0) / (self.z.1 - self.z.0);
        hit_record.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);
        true
    }
}
