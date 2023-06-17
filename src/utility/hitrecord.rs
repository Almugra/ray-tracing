use glam::Vec3;

use super::ray::Ray;

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            reverse(outward_normal)
        }
    }
}

pub fn reverse(vec: Vec3) -> Vec3 {
    Vec3 {
        x: -vec.x,
        y: -vec.y,
        z: -vec.z,
    }
}
