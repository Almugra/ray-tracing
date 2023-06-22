use std::sync::Arc;

use crate::{hit::hittable::Hittable, materials::Material, Point3};

pub struct MovingSphere {
    pub center: (Point3, Point3),
    pub time: (f32, f32),
    pub radius: f32,
    pub material: Option<Arc<dyn Material>>,
}

unsafe impl Send for MovingSphere {}
unsafe impl Sync for MovingSphere {}

impl MovingSphere {
    #[allow(unused)]
    pub fn new(
        center: (Point3, Point3),
        time: (f32, f32),
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center,
            time,
            radius,
            material: Some(material),
        }
    }

    fn center(&self, time: f32) -> Point3 {
        self.center.0
            + ((time - self.time.0) / (self.time.1 - self.time.0) * (self.center.1 - self.center.0))
    }
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut crate::hit::hitrecord::HitRecord,
    ) -> bool {
        let offset_vector = ray.origin - self.center(ray.time);

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
        let outward_normal = (hit_record.point - self.center(ray.time)) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = self.material.clone();

        true
    }
}
