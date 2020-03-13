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
    pub fn new(width: usize, data: Vec<Cell>) -> GameMap {
        let height = data.len() / width;
        assert!(width * height == data.len());

        GameMap {
            width,
            height,
            data,
        }
    }

    pub fn xy_to_id(&self, x: usize, y: usize) -> CellId {
        CellId::new(y * self.width + x)
    }

    pub fn id_to_xy(&self, id: CellId) -> (usize, usize) {
        let x = id.raw() % self.width;
        let y = (id.raw() - x) / self.width;
        (x, y)
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
            width: w,
            height: h,
            data: Vec::new(),
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
            width: w,
            height: h,
            data: Vec::new(),
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
            width: w,
            height: h,
            data: Vec::new(),
        };

        let cell_id = CellId::new(22);
        let (x, y) = map.id_to_xy(cell_id);

        assert_eq!(cell_id, map.xy_to_id(x, y));
    }
}
