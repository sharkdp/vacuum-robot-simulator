use math::Scalar;
use geometry::{Pose, Vector};
use sensor::laserscanner::Scan;

pub const SIZE: usize = 111;
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

    pub fn update(&mut self, pose: &Pose, scan: &Scan) {
        use self::CellState::*;

        for &m in scan.iter() {
            let p = m.to_vector(pose);

            match GridMap::indices_from_pos(p) {
                Some((r, c)) => {
                    let cell: &mut CellState = &mut self.cells[r][c];
                    *cell = match *cell {
                        Occupied(count) => Occupied(count + 1),
                        Freespace => Occupied(1),
                        Void => Occupied(1)
                    };
                },
                _ => {}
            }
        }
    }

    fn index_from_dist(dist: Scalar) -> Option<usize> {
        let size = SIZE as Scalar;
        let c = (dist / CELL_LENGTH + size / 2.0) as i32;

        if c < 0 || c >= (SIZE as i32) {
            None
        } else {
            Some(c as usize)
        }
    }

    fn indices_from_pos(pos: Vector) -> Option<(usize, usize)> {
        GridMap::index_from_dist(pos.y)
            .and_then(|r| GridMap::index_from_dist(pos.x).map(|c| (r, c)))
    }

    pub fn cell_state(&self, r: usize, c: usize) -> Option<&CellState> {
        self.cells.get(r)
                  .and_then(|row| row.get(c))
    }
}
