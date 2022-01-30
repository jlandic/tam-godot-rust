use bevy_ecs::prelude::*;
use gdnative::core_types::Vector2;

use crate::data::tiles::TileId;
use crate::ecs::components::*;
use crate::ecs::resources::DebugConfig;
use crate::engine::resources::TileSetIdMapper;
use crate::engine::shared::DEBUG_TILE_MAP_COMPONENT_NAME;
use crate::engine::{tree, Owner};

pub fn debug_show_paths(
    config: Res<DebugConfig>,
    owner: Res<Owner>,
    tile_set_id_mapper: Res<TileSetIdMapper>,
    query: Query<&Path>,
) {
    if !config.draw_path {
        return;
    }

    if let Some(tile_map) =
        unsafe { tree::get_tile_map(DEBUG_TILE_MAP_COMPONENT_NAME, &owner.assume_unique(), true) }
    {
        for path in query.iter() {
            for point in &path.points {
                tile_map.set_cell(
                    point.x as i64,
                    point.y as i64,
                    tile_set_id_mapper.get_tile_set_id(TileId::Empty),
                    false,
                    false,
                    false,
                    Vector2::new(0., 0.),
                );
            }
        }
    }
}
