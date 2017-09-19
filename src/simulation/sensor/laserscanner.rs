use std::cmp::Ordering;

use math::{Scalar, Angle, PI};
use geometry::{Target, Point, Pose, Ray, Line};
use pointcloud::PointCloud;

pub struct LaserScanner {
    pub num_columns: u32,
    // max_range: Scalar,
    // range_noise: Scalar,
    // angle_noise: Angle
}

impl LaserScanner {
    pub fn scan(&self, pose: &Pose, targets: &[Line]) -> PointCloud {
        let mut cloud = PointCloud::empty();

        // Comparison function to find the closest point
        let distance = |p: &Point| { (p.pos - pose.position).length() };

        // Raycasting
        for col in 0 .. self.num_columns {
            let angle = pose.heading.angle() + self.column_to_angle(col);
            let ray = Ray::from_angle(pose.position, angle);

            let mut points = vec!();
            for target in targets.iter() {
                let mut candidates = target.intersect(&ray);
                points.append(&mut candidates);
            }

            // Only take the point closest to the sensor
            let closest = points.iter().min_by(
                |p1, p2| distance(p1).partial_cmp(&distance(p2))
                                     .unwrap_or(Ordering::Equal));

            if let Some(p) = closest {
                cloud.add(*p);
            }
        }

        cloud
    }

    fn column_to_angle(&self, column: u32) -> Angle {
        (column as Scalar) / (self.num_columns as Scalar) * 2.0 * PI
    }
}
