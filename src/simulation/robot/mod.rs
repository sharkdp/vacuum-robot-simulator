use geometry::Pose;
use simulation::sensor::laserscanner::LaserScanner;

pub struct Robot {
    pub pose: Pose,
    pub laser_scanner: LaserScanner,
}
