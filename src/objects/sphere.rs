use glam::Vec3;

use crate::{hit::{hittable::Hittable, hitrecord::HitRecord}, ray::Ray};

#[derive(Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let offset_vector = ray.origin - self.center;

        // Coefficients for the quadratic formula
        let a = ray.direction.length_squared();
        let half_b = offset_vector.dot(ray.direction);
        let c = offset_vector.length_squared() - self.radius.powi(2);

        let discriminant = half_b * half_b - a * c;

        // if discriminant is negative, there's no real roots, hence no intersection
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Find nearest root in acceptable range [t_min, t_max]
        let mut nearest_root = (-half_b - sqrt_discriminant) / a;
        if nearest_root < t_min || t_max < nearest_root {
            nearest_root = (-half_b + sqrt_discriminant) / a;
            if nearest_root < t_min || t_max < nearest_root {
                return false;
            }
        }

        // Validated intersection at this point
        hit_record.t = nearest_root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        true
    }
}
