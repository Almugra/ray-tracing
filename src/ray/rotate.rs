use std::sync::Arc;

use crate::hit::hittable::Hittable;

use super::Ray;

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
}

impl RotateY {
    #[allow(unused)]
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        Self {
            object,
            sin_theta,
            cos_theta,
        }
    }
}

impl Hittable for RotateY {
    fn hit(
        &self,
        ray: &super::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut crate::hit::hitrecord::HitRecord,
    ) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if !self.object.hit(&rotated_ray, t_min, t_max, hit_record) {
            return false;
        }

        let mut p = hit_record.point;
        let mut normal = hit_record.normal;

        p[0] = self.cos_theta * hit_record.point[0] + self.sin_theta * hit_record.point[2];
        p[2] = -self.sin_theta * hit_record.point[0] + self.cos_theta * hit_record.point[2];

        normal[0] = self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
        normal[2] = -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

        hit_record.point = p;
        hit_record.set_face_normal(&rotated_ray, normal);

        true
    }
}
