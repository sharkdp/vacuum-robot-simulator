use math::Angle;
use super::Vector;

#[derive(Debug, Clone)]
pub struct Pose {
    pub position: Vector,
    pub heading: Vector
}

impl Default for Pose {
    fn default() -> Pose {
        Pose::new(Vector::new(0.0, 0.0), 0.0)
    }
}

impl Pose {
    pub fn new(position: Vector, heading: Angle) -> Pose {
        Pose { position, heading: Vector::from_angle(heading) }
    }
}
