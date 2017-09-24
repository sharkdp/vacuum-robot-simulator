use math::Scalar;
use sensor::laserscanner::Scan;

pub const SIZE: usize = 100;
pub const CELL_LENGTH: Scalar = 0.25;

#[derive(Debug, Copy, Clone)]
pub enum CellState {
    Occupied(u32),
    Freespace,
    Void
}

impl Default for CellState {
    fn default() -> CellState {
        CellState::Void
    }
}

pub struct GridMap {
    cells: [[CellState; SIZE]; SIZE]
}

impl Default for GridMap {
    fn default() -> GridMap {
        GridMap { cells: [[CellState::default(); SIZE]; SIZE] }
    }
}

impl GridMap {
    pub fn clear(&mut self) {
        self.cells = [[CellState::default(); SIZE]; SIZE];
    }

    pub fn update(&mut self, scan: &Scan) {
        use self::CellState::*;

        for &m in scan.iter() {
            let v = m.to_vector();

            let r = GridMap::index_from_pos(v.y);
            let c = GridMap::index_from_pos(v.x);

            let cell: &mut CellState = &mut self.cells[r][c];
            *cell = match *cell {
                Occupied(count) => Occupied(count + 1),
                Freespace => Occupied(1),
                Void => Occupied(1)
            }
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

    pub fn cell_state(&self, r: usize, c: usize) -> Option<&CellState> {
        self.cells.get(r)
                  .and_then(|row| row.get(c))
    }
}
