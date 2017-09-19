pub mod gridmap;

use self::gridmap::GridMap;
use pointcloud::PointCloud;

pub struct Controller {
    gridmap: GridMap
}

impl Default for Controller {
    fn default() -> Controller {
        Controller { gridmap: GridMap::default() }
    }
}

impl Controller {

    pub fn cycle(&mut self, pointcloud: &PointCloud) {
        self.gridmap.clear(); // TODO

        self.gridmap.update(pointcloud);
        self.gridmap.print();
    }
}
