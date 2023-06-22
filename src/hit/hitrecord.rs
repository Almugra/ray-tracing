use std::sync::Arc;

use crate::{materials::Material, ray::Ray, Point3, Vector3};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
