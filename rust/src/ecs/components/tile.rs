use crate::data::tiles::TileId;
use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct Tile {
    pub id: TileId,
}

impl Tile {
    pub fn new(tile_id: TileId) -> Self {
        Self { id: tile_id }
    }
}
