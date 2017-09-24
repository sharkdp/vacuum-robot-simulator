pub mod gridmap;

use self::gridmap::GridMap;
use sensor::laserscanner::Scan;

pub struct Controller {
    pub gridmap: GridMap
}

impl Default for Controller {
    fn default() -> Controller {
        Controller { gridmap: GridMap::default() }
    }
}

impl Controller {
    pub fn cycle(&mut self, scan: &Scan) {
        self.gridmap.clear(); // TODO

        self.gridmap.update(scan);
    }
}
