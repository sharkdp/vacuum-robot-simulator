use simulation::sensor::laserscanner::LaserScanner;
use geometry::Pose;

pub struct Robot {
    pose: Pose,
    laserscanner: LaserScanner
}
