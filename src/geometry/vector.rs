use std::fmt;
use std::ops;
use std::cmp;

use math::{Scalar, Angle};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: Scalar,
    pub y: Scalar
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, s: Scalar) -> Vector {
        Vector::new(self.x * s, self.y * s)
    }
}

impl cmp::PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector {
    pub fn new(x: Scalar, y: Scalar) -> Vector {
        Vector { x, y }
    }

    pub fn from_angle(angle: Angle) -> Vector {
        // 0° is in forward direction (along Y-axis)
        Vector { x: -angle.sin(), y: angle.cos() }
    }

    pub fn length(&self) -> Scalar {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot(&self, q: Vector) -> Scalar {
        self.x * q.x + q.y * self.y
    }

    pub fn cross(&self, q: Vector) -> Scalar {
        self.x * q.y - q.x * self.y
    }

    pub fn angle(&self) -> Scalar {
        -self.x.atan2(self.y)
    }

    pub fn rotate(&self, angle: Angle) -> Vector {
        let c = angle.cos();
        let s = angle.sin();
        Vector::new(c * self.x - s * self.y, s * self.x + c * self.y)
    }
}
