use rand::Rng;

use crate::{
    hit::{hitlist::HitList, hitrecord::HitRecord, hittable::Hittable},
    Color, Point3, Vector3,
};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

/// Calculate the color of a ray
pub fn ray_color(ray: &Ray, world: &HitList, depth: isize) -> Color {
    let mut hit_record = HitRecord::default();

    if depth <= 0 {
        return Vector3::ZERO;
    }

    if world.hit(ray, 0.001, f32::MAX, &mut hit_record) {
        let Some(material) = hit_record.material.clone() else {
            unreachable!()
        };

        if let Some((attenuation, scattered)) = material.scatter(ray, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Vector3::ZERO;
    }

    let unit_direction = unit_vec(ray.direction);
    let t = (unit_direction.y + 1.0) * 0.5;
    Vector3::ONE * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0)
}

pub fn unit_vec(v: Vector3) -> Vector3 {
    v / v.length()
}

fn random_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();
    let mut gen_range = || rng.gen_range(-1.0..1.0);
    loop {
        let p = Vector3::new(gen_range(), gen_range(), gen_range());
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vector3 {
    unit_vec(random_in_unit_sphere())
}
