pub mod gridmap;

use self::gridmap::GridMap;
use pointcloud::PointCloud;

pub struct Controller {
    gridmap: GridMap
}

impl Controller {
    pub fn new() -> Controller {
        Controller { gridmap: GridMap::new() }
    }

    pub fn cycle(&mut self, pointcloud: &PointCloud) {
        self.gridmap.clear(); // TODO

        self.gridmap.update(pointcloud);
        self.gridmap.print();
    }
}
