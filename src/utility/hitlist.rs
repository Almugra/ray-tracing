use super::{hitrecord::HitRecord, hittable::Hittable, ray::Ray};

#[derive(Default)]
pub struct HitList<T>
where
    T: Hittable,
{
    pub objects: Vec<T>,
}

impl<T: Hittable> HitList<T> {
    pub fn push(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HitList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }

        hit_anything
    }
}
