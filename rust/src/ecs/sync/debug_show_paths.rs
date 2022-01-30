use bevy_ecs::prelude::*;
use gdnative::core_types::Vector2;

use crate::data::geo::Map;
use crate::data::tiles::TileId;
use crate::ecs::components::*;
use crate::engine::resources::TileSetIdMapper;
use crate::engine::shared::DEBUG_TILE_MAP_COMPONENT_NAME;
use crate::engine::{tree, Owner};

#[allow(dead_code)]
pub fn debug_show_paths(
    owner: Res<Owner>,
    map: Res<Map>,
    tile_set_id_mapper: Res<TileSetIdMapper>,
    query: Query<&Path>,
) {
    if let Some(tile_map) =
        unsafe { tree::get_tile_map(DEBUG_TILE_MAP_COMPONENT_NAME, &owner.assume_unique(), true) }
    {
        for x in 0..map.size().x {
            for y in 0..map.size().y {
                tile_map.set_cell(
                    x as i64,
                    y as i64,
                    -1,
                    false,
                    false,
                    false,
                    Vector2::new(0., 0.),
                );
            }
        }

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
