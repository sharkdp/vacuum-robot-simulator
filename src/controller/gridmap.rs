use math::Scalar;
use pointcloud::PointCloud;

const SIZE: usize = 50;
const CELL_LENGTH: Scalar = 0.5;

#[derive(Debug, Copy, Clone)]
struct Cell {
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

    // TODO
    pub fn print(&self) {
        for r in 0 .. SIZE {
            for c in 0 .. SIZE {
                print!("{}", if self.cells[r][c].count > 0 { "#" } else { " " });
            }
            println!();
        }
        println!("---------------");
    }
}
