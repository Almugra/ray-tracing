use crate::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}
