use std::collections::HashMap;

use gdnative::api::TileSet;
use gdnative::prelude::*;

use crate::data::tiles::TileId;

pub struct TileSetIdMapper {
    map: HashMap<TileId, i64>,
}

impl TileSetIdMapper {
    pub fn new(tile_set: Ref<TileSet, Unique>) -> Self {
        let mut map = HashMap::<TileId, i64>::new();
        TileId::iterator().for_each(|tile| {
            map.insert(tile, tile_set.find_tile_by_name(tile.id()));
        });

        Self { map }
    }

    pub fn get_tile_set_id(&self, tile: TileId) -> i64 {
        self.map[&tile]
    }
}
