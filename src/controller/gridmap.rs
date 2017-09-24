use math::Scalar;
use pointcloud::PointCloud;

pub const SIZE: usize = 100;
pub const CELL_LENGTH: Scalar = 0.25;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    count: u32
}

impl Cell {
    fn new() -> Cell {
        Cell { count: 0 }
    }
}

pub struct GridMap {
    cells: [[Cell; SIZE]; SIZE]
}

impl Default for GridMap {
    fn default() -> GridMap {
        GridMap { cells: [[Cell::new(); SIZE]; SIZE] }
    }
}

impl GridMap {
    pub fn clear(&mut self) {
        self.cells = [[Cell::new(); SIZE]; SIZE];
    }

    pub fn update(&mut self, pointcloud: &PointCloud) {
        for &p in pointcloud.iter() {
            let r = GridMap::index_from_pos(p.pos.y);
            let c = GridMap::index_from_pos(p.pos.x);
            self.cells[r][c].count += 1;
        }
    }

    fn index_from_pos(pos: Scalar) -> usize {
        let size = SIZE as Scalar;
        let c = (pos / CELL_LENGTH + size / 2.0) as i32;

        if c < 0 {
            0
        } else if c >= (SIZE as i32) {
            SIZE - 1
        } else {
            c as usize
        }
    }

    pub fn get_count(&self, r: usize, c: usize) -> Option<u32> {
        self.cells.get(r)
                  .and_then(|row| row.get(c))
                  .map(|cell| cell.count)
    }
}
