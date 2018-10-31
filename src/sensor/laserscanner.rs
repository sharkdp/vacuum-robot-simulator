/// Types for dealing with laser scanner measurements
use std::slice::Iter;

use geometry::{Point, Pose, Vector};
use math::{Angle, Scalar};
use pointcloud::PointCloud;

/// A single measurement (distance reading) of a laser scanner.
#[derive(Debug, Clone, Copy)]
pub struct Measurement {
    pub angle: Angle,
    pub distance: Scalar,
}

impl Measurement {
    pub fn new(angle: Angle, distance: Scalar) -> Measurement {
        Measurement { angle, distance }
    }

    pub fn to_vector(&self, pose: &Pose) -> Vector {
        let direction = Vector::from_angle(pose.heading + self.angle);
        pose.position + direction * self.distance
    }
}

/// A full 360° scan from a laser scanner.
pub struct Scan {
    measurements: Vec<Measurement>,
}

impl Scan {
    pub fn empty() -> Scan {
        Scan {
            measurements: Vec::new(),
        }
    }

    pub fn add(&mut self, m: Measurement) {
        self.measurements.push(m);
    }

    pub fn iter(&self) -> Iter<Measurement> {
        self.measurements.iter()
    }

    pub fn to_pointcloud(&self, pose: &Pose) -> PointCloud {
        PointCloud::new(
            self.measurements
                .iter()
                .map(|m| m.to_vector(pose))
                .map(Point::from_vector)
                .collect(),
        )
    }
}
