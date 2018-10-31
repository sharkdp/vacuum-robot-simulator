use super::Vector;
use math::Angle;

#[derive(Debug, Clone)]
pub struct Pose {
    pub position: Vector,
    pub heading: Angle,
}

impl Default for Pose {
    fn default() -> Pose {
        Pose::new(Vector::new(0.0, 0.0), 0.0)
    }
}

impl Pose {
    pub fn new(position: Vector, heading: Angle) -> Pose {
        Pose { position, heading }
    }
}
