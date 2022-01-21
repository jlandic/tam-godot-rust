use bevy_ecs::prelude::*;
use gdnative::core_types::Vector2;

use crate::data::geo::Map;
use crate::engine::shared::FOW_TILE_MAP_COMPONENT_NAME;
use crate::engine::{tree, Owner};

pub fn sync_fow(owner: Res<Owner>, map: Res<Map>) {
    if let Some(tile_map) =
        unsafe { tree::get_tile_map(FOW_TILE_MAP_COMPONENT_NAME, &owner.assume_unique(), true) }
    {
        for tile in &map.revealed {
            tile_map.set_cell(
                tile.x as i64,
                tile.y as i64,
                -1,
                false,
                false,
                false,
                Vector2::new(0., 0.),
            );
        }
    }
}
