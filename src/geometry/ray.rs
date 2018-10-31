use super::Vector;
use math::Angle;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn from_angle(origin: Vector, angle: Angle) -> Ray {
        Ray {
            origin,
            direction: Vector::from_angle(angle),
        }
    }
}
