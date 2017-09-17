use simulation::sensor::laserscanner::LaserScanner;
use geometry::Pose;

pub struct Robot {
    pub pose: Pose,
    pub laser_scanner: LaserScanner
}
