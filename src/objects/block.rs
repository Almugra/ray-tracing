use std::sync::Arc;

use crate::{
    hit::{hitlist::HitList, hittable::Hittable},
    materials::Material,
    Point3,
};

use super::{xy_rectangle::XYRectangle, xz_rectangle::XZRectangle, yz_rectangle::YZRectangle};

pub struct Block {
    sides: HitList,
}

impl Block {
    #[allow(unused)]
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Self {
        let mut list = HitList::default();

        list.push(Arc::new(XYRectangle::new(
            (p0.x, p1.x),
            (p0.y, p1.y),
            p1.z,
            material.clone(),
        )));
        list.push(Arc::new(XYRectangle::new(
            (p0.x, p1.x),
            (p0.y, p1.y),
            p0.z,
            material.clone(),
        )));

        list.push(Arc::new(XZRectangle::new(
            (p0.x, p1.x),
            (p0.z, p1.z),
            p1.y,
            material.clone(),
        )));
        list.push(Arc::new(XZRectangle::new(
            (p0.x, p1.x),
            (p0.z, p1.z),
            p0.y,
            material.clone(),
        )));

        list.push(Arc::new(YZRectangle::new(
            (p0.y, p1.y),
            (p0.z, p1.z),
            p1.x,
            material.clone(),
        )));
        list.push(Arc::new(YZRectangle::new(
            (p0.y, p1.y),
            (p0.z, p1.z),
            p0.x,
            material,
        )));

        Self { sides: list }
    }
}

impl Hittable for Block {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut crate::hit::hitrecord::HitRecord,
    ) -> bool {
        self.sides.hit(ray, t_min, t_max, hit_record)
    }
}
