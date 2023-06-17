use glam::Vec3;

pub struct Viewport {
    pub height: f32,
    pub width: f32,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Viewport {
    pub fn new(aspect_ratio: f32, height: f32, focal_length: f32, origin: Vec3) -> Self {
        let width = aspect_ratio * height;
        let horizontal = Vec3::new(width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            height,
            width,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
