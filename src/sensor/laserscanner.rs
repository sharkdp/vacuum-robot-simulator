/// Types for dealing with laser scanner measurements

use std::slice::Iter;

use math::{Scalar, Angle};
use geometry::{Vector, Point, Pose};
use pointcloud::PointCloud;

/// A single measurement (distance reading) of a laser scanner.
#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    angle: Angle,
    distance: Scalar
}

impl Measurement {
    pub fn new(angle: Angle, distance: Scalar) -> Measurement {
        Measurement { angle, distance }
    }

    pub fn to_vector(&self, pose: &Pose) -> Vector {
        let direction = pose.heading.rotate(self.angle);
        pose.position + direction * self.distance
    }
}

/// A full 360° scan from a laser scanner.
pub struct Scan {
    measurements: Vec<Measurement>
}

impl Scan {
    pub fn empty() -> Scan {
        Scan { measurements: Vec::new() }
    }

    pub fn add(&mut self, m: Measurement) {
        self.measurements.push(m);
    }

    pub fn iter(&self) -> Iter<Measurement> {
        self.measurements.iter()
    }

    pub fn to_pointcloud(&self, pose: &Pose) -> PointCloud {
        PointCloud::new(
            self.measurements.iter()
                             .map(|m| m.to_vector(pose))
                             .map(Point::from_vector)
                             .collect()
       )
    }
}
