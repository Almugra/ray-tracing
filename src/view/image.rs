pub struct Image {
    pub width: f32,
    pub height: f32,
}

impl Image {
    pub fn new(aspect_ratio: f32, width: f32) -> Self {
        Self {
            width,
            height: width / aspect_ratio,
        }
    }
}
