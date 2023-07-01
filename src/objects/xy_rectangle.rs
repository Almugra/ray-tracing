use glam::Vec3;

use crate::{hit::hittable::Hittable, materials::Material};
use std::sync::Arc;

pub struct XYRectangle {
    pub material: Option<Arc<dyn Material>>,
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub k: f32,
}

impl XYRectangle {
    #[allow(unused)]
    pub fn new(x: (f32, f32), y: (f32, f32), k: f32, material: Arc<dyn Material>) -> Self {
        Self {
            material: Some(material),
            x,
            y,
            k,
        }
    }
}

unsafe impl Send for XYRectangle {}
unsafe impl Sync for XYRectangle {}

impl Hittable for XYRectangle {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut crate::hit::hitrecord::HitRecord,
    ) -> bool {
        let t = (self.k - ray.origin.z) / (ray.direction.z);
        if t < t_min || t > t_max {
            return false;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return false;
        }
        hit_record.uv.0 = (x - self.x.0) / (self.x.1 - self.x.0);
        hit_record.uv.1 = (y - self.y.0) / (self.y.1 - self.y.0);
        hit_record.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);
        true
    }
}
