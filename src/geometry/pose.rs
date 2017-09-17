use math::Angle;
use super::Vector;

pub struct Pose {
    pub position: Vector,
    pub heading: Vector
}

impl Pose {
    pub fn new(position: Vector, heading: Angle) -> Pose {
        Pose { position, heading: Vector::from_angle(heading) }
    }
}
