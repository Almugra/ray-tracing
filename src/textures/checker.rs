use crate::Color;

use super::{solid::SolidColor, texture::Texture};

pub struct Checker {
    pub odd: SolidColor,
    pub even: SolidColor,
}

impl Checker {
    #[allow(unused)]
    pub fn new(c1: Color, c2: Color) -> Self {
        Self {
            odd: SolidColor::new(c1),
            even: SolidColor::new(c2),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f32, v: f32, p: &crate::Point3) -> crate::Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
