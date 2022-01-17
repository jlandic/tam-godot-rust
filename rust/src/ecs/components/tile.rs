use crate::data::tiles::TileId;

#[derive(Debug)]
pub struct Tile {
    pub id: String,
}

impl Tile {
    pub fn new(tile_id: TileId) -> Self {
        Self {
            id: tile_id.id().to_string(),
        }
    }
}
