use crate::id::*;
use serde::{Deserialize, Serialize};
use std::ops::Index;

pub type CellId = Id<Cell>;
pub type GameMapBuilder = MapBuilder<GameMap>;

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
    data: Vec<Cell>,
    width: usize,
    height: usize,
}

impl GameMap {
    pub fn xy_to_id(&self, x: usize, y: usize) -> CellId {
        CellId::new(y * self.width + x)
    }
}

impl Index<CellId> for GameMap {
    type Output = Cell;

    fn index(&self, id: CellId) -> &Self::Output {
        &self.data[id.raw()]
    }
}
