use std::ops::{Add, Div, Mul, Sub};

use super::ray::Ray;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn all(val: f64) -> Self {
        Self {
            x: val,
            y: val,
            z: val,
        }
    }

    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (255.999 * self.x) as usize,
            (255.999 * self.y) as usize,
            (255.999 * self.z) as usize
        );
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn unit_vec(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            panic!("cannot divide by 0");
        }
        self::Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self::Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self::Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self::Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self::Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

pub fn ray_color(r: Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vec();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::all(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
