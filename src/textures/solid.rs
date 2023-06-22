use crate::Color;

use super::texture::Texture;

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        SolidColor { color: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f32, _: f32, _: &crate::Point3) -> Color {
        self.color
    }
}
