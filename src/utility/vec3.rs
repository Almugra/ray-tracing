pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (255.999 * self.x) as usize,
            (255.999 * self.y) as usize,
            (255.999 * self.z) as usize
        );
    }
}
