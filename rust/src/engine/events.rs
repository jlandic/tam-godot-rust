use crate::data::{geo::Vec2, tiles::TileId};
use gdnative::prelude::*;

#[derive(Default)]
pub struct MovementInput {
    pub direction: Vec2,
}
impl From<Vector2> for MovementInput {
    fn from(godot_vector: Vector2) -> Self {
        Self {
            direction: Vec2::from(godot_vector),
        }
    }
}

#[derive(Default)]
pub struct MapTileUpdated {
    pub position: Vec2,
    pub tile: TileId,
}
