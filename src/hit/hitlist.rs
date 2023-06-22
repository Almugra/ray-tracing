use std::sync::Arc;

use crate::ray::Ray;

use super::{hitrecord::HitRecord, hittable::Hittable};

#[derive(Default, Clone)]
pub struct HitList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HitList {
    pub fn push(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }

        hit_anything
    }
}
