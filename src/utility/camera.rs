use glam::Vec3;

use self::{image::Image, viewport::Viewport};

use super::ray::Ray;
pub mod image;
pub mod viewport;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image: Image,
    pub viewport: Viewport,
    pub focal_length: f32,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, width: f32, focal_length: f32, viewport_height: f32) -> Self {
        let image = Image::new(aspect_ratio, width);
        let origin = Vec3::default();
        let viewport = Viewport::new(aspect_ratio, viewport_height, 1.0, origin);
        Self {
            image,
            viewport,
            focal_length,
            origin,
            aspect_ratio,
        }
    }

    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.viewport.lower_left_corner
                + self.viewport.horizontal * u
                + self.viewport.vertical * v
                - self.origin,
        }
    }
}
