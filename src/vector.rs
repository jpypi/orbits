use std::ops::{Add, Mul};

pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0 }
    }
}

impl<'a> Add for &'a Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, f: f32) -> Vector {
        Vector { x: self.x * f, y: self.y * f }
    }
}
