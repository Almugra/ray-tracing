use std::sync::Arc;

use crate::{
    textures::{solid::SolidColor, texture::Texture},
    Color,
};

use super::Material;

pub struct DiffuseLight {
    pub emit: SolidColor,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        Self {
            emit: SolidColor::new(c),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _: &crate::ray::Ray,
        _: &crate::hit::hitrecord::HitRecord,
    ) -> Option<(crate::Color, crate::ray::Ray)> {
        None
    }
    fn emitted(&self, u: f32, v: f32, p: crate::Point3) -> Color {
        self.emit.value(u, v, &p)
    }
}
