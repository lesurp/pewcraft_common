use crate::id::*;
use log::debug;
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::ops::Index;

pub type GameMapId = Id<GameMap>;
pub type CellId = Id<Cell>;
pub type GameMapBuilder = MapBuilder<GameMap>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct TeamId(usize);

impl TeamId {
    pub fn new(id: usize) -> Self {
        TeamId(id)
    }

    pub fn raw(self) -> usize {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Team(pub String, pub HashSet<CellId>);

impl CellId {
    fn invalid() -> CellId {
        CellId::new(std::usize::MAX)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CellAttibute {
    None,
    Wall,
    Hole,
    Water,
    // etc.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cell {
    height: i32,
    attribute: CellAttibute,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameMap {
    pub name: String,
    pub data: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub teams: Vec<Team>,
}

// Used internally for the A* path computation
#[derive(Eq)]
struct Node {
    real_cost: u32,
    heuristic: u32,
    curr_cell: CellId,
}

impl Node {
    fn cost(&self) -> u32 {
        self.real_cost + self.heuristic
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost().cmp(&other.cost())
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}

impl GameMap {
    pub fn check_validity(&self) -> Result<(), ()> {
        if self.height * self.width != self.data.len() {
            return Err(());
        }

        debug!(
            "Map named `{}` checked properly (size {}x{})",
            self.name, self.width, self.height
        );
        Ok(())
    }

    pub fn xy_to_id(&self, x: usize, y: usize) -> CellId {
        CellId::new(y * self.width + x)
    }

    pub fn id_to_xy(&self, id: CellId) -> (usize, usize) {
        let x = id.raw() % self.width;
        let y = (id.raw() - x) / self.width;
        (x, y)
    }

    pub fn id_to_xy_i32(&self, id: CellId) -> (i32, i32) {
        let x = id.raw() % self.width;
        let y = (id.raw() - x) / self.width;
        (x as i32, y as i32)
    }

    fn surrounding_cells(&self, c: CellId) -> [CellId; 4] {
        let (x, y) = self.id_to_xy(c);

        // NOTE: those values WILL underflow for the first row
        // however those ids are unsigned, meaning that they will be invalidated when using
        // is_valid_cell anyway
        [
            if x + 1 < self.width {
                self.xy_to_id(x + 1, y)
            } else {
                CellId::invalid()
            },
            if y + 1 < self.height {
                self.xy_to_id(x, y + 1)
            } else {
                CellId::invalid()
            },
            if x > 0 {
                self.xy_to_id(x - 1, y)
            } else {
                CellId::invalid()
            },
            if y > 0 {
                self.xy_to_id(x, y - 1)
            } else {
                CellId::invalid()
            },
        ]
    }

    fn is_valid_cell(&self, c: CellId) -> bool {
        self.data.len() > c.raw()
    }

    pub fn distance(&self, a: CellId, b: CellId) -> u32 {
        let (ax, ay) = self.id_to_xy(a);
        let (bx, by) = self.id_to_xy(b);
        ((ax as i32 - bx as i32).abs() + (ay as i32 - by as i32).abs()) as u32
    }

    pub fn can_move_to(&self, start: CellId, end: CellId, swiftness: i32) -> bool {
        let mut nodes = BinaryHeap::new();
        nodes.push(Reverse(Node {
            real_cost: 0,
            heuristic: self.distance(start, end),
            curr_cell: start,
        }));

        loop {
            let n = if let Some(n) = nodes.pop() {
                n.0
            } else {
                // No path possible (should never happen though...)
                return false;
            };

            // our two exit conditions (arrived, or too long)
            if swiftness < n.cost() as i32 {
                return false;
            } else if end == n.curr_cell {
                return true;
            }

            let prev_cell = &self[n.curr_cell];
            for neighbor in self.surrounding_cells(n.curr_cell).iter() {
                let neighbor = *neighbor;
                if !self.is_valid_cell(neighbor) {
                    continue;
                }

                let next_cell = &self[neighbor];

                let additional_cost =
                    std::cmp::max(0, next_cell.height - prev_cell.height) as u32 + 1;

                nodes.push(Reverse(Node {
                    real_cost: n.real_cost + additional_cost,
                    heuristic: self.distance(neighbor, end),
                    curr_cell: neighbor,
                }));
            }
        }
    }
}

impl Index<CellId> for GameMap {
    type Output = Cell;

    fn index(&self, id: CellId) -> &Self::Output {
        &self.data[id.raw()]
    }
}

#[cfg(test)]
mod test {
    use super::CellId;
    use super::GameMap;

    #[test]
    fn test_xy_to_id() {
        let w = 4;
        let h = 6;
        let map = GameMap {
            name: "".to_owned(),
            width: w,
            height: h,
            data: Vec::new(),
            starting_cells_1: Default::default(),
            starting_cells_2: Default::default(),
        };

        let x = 1;
        let y = 3;

        let cell_id = map.xy_to_id(x, y);

        assert_eq!(cell_id.raw(), 13);
    }

    #[test]
    fn test_id_to_xy() {
        let w = 4;
        let h = 6;
        let map = GameMap {
            name: "".to_owned(),
            width: w,
            height: h,
            data: Vec::new(),
            starting_cells_1: Default::default(),
            starting_cells_2: Default::default(),
        };

        let cell_id = CellId::new(22);
        let (x, y) = map.id_to_xy(cell_id);

        assert_eq!(x, 2);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_id_consistency() {
        let w = 4;
        let h = 6;
        let map = GameMap {
            name: "".to_owned(),
            width: w,
            height: h,
            data: Vec::new(),
            starting_cells_1: Default::default(),
            starting_cells_2: Default::default(),
        };

        let cell_id = CellId::new(22);
        let (x, y) = map.id_to_xy(cell_id);

        assert_eq!(cell_id, map.xy_to_id(x, y));
    }
}
