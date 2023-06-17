use glam::Vec3;

use crate::hit::{hitrecord::HitRecord, hittable::Hittable, HitList};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

/// Calculate the color of a ray
pub fn ray_color<T: Hittable>(ray: &Ray, world: &HitList<T>) -> Vec3 {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.0, f32::MAX, &mut hit_record) {
        return (hit_record.normal + Vec3::ONE) * 0.5;
    }

    let unit_direction = unit_vec(ray.direction);
    let t = (unit_direction.y + 1.0) * 0.5;
    Vec3::ONE * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0)
}

pub fn unit_vec(v: Vec3) -> Vec3 {
    v / v.length()
}
