use crate::data::geo::{Vec2, Vec2Unsigned};
use crate::data::tiles::TileId;

pub struct Map {
    pub tiles: Vec<TileId>,
    pub size: Vec2Unsigned,
}

impl Map {
    pub fn is_colliding(&self, position: Vec2) -> bool {
        self.tiles[self.xy_index(position)] == TileId::Wall
    }

    pub fn xy_index(&self, position: Vec2) -> usize {
        position.index(self.size.x as i32) as usize
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: Vec::new(),
            size: Vec2Unsigned::zero(),
        }
    }
}
