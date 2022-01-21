use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::data::geo::Map;
use crate::data::tiles::TileId;
use crate::ecs::components::*;
use crate::engine::resources::TileSetIdMapper;
use crate::engine::shared::OCCLUSION_TILE_MAP_COMPONENT_NAME;
use crate::engine::{tree, Owner};

pub fn update_viewshed(
    owner: Res<Owner>,
    tile_set_id_mapper: Res<TileSetIdMapper>,
    map: Res<Map>,
    query: Query<(&Viewshed, With<Player>)>,
) {
    if let Some(tile_map) = unsafe {
        tree::get_tile_map(
            OCCLUSION_TILE_MAP_COMPONENT_NAME,
            &owner.assume_unique(),
            true,
        )
    } {
        for i in 0..map.size().x {
            for j in 0..map.size().y {
                tile_map.set_cell(
                    i as i64,
                    j as i64,
                    tile_set_id_mapper.get_tile_set_id(TileId::Empty),
                    false,
                    false,
                    false,
                    Vector2::new(0., 0.),
                );
            }
        }

        for (viewshed, _) in query.iter() {
            for point in &viewshed.visible_tiles {
                tile_map.set_cell(
                    point.x as i64,
                    point.y as i64,
                    -1,
                    false,
                    false,
                    false,
                    Vector2::new(0., 0.),
                );
            }
        }
    }
}
