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

    pub fn to_vector(&self) -> Vector {
        let x = self.distance * self.angle.sin();
        let y = self.distance * self.angle.cos();
        Vector::new(x, y)
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
        let mut cloud = PointCloud::empty();
        for m in self.measurements.iter() {
            let direction = pose.heading.rotate(m.angle);
            let p = pose.position + direction * m.distance;
            cloud.add(Point::from_vector(p));
        }
        cloud
    }
}
