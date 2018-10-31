pub mod gridmap;

use geometry::Pose;
use sensor::laserscanner::Scan;

use self::gridmap::GridMap;

pub struct Controller {
    pub gridmap: GridMap,
    pub pose_estimate: Pose,
}

impl Default for Controller {
    fn default() -> Controller {
        Controller {
            gridmap: GridMap::default(),
            pose_estimate: Pose::default(),
        }
    }
}

impl Controller {
    pub fn cycle(&mut self, scan: &Scan, pose_todo: &Pose) {
        // self.gridmap.clear();

        // TODO: this is cheating
        self.pose_estimate = pose_todo.clone();

        self.gridmap.update(&self.pose_estimate, scan);
    }
}
