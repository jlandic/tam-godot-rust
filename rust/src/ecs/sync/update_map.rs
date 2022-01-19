use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::ecs::components::{Transform, *};
use crate::engine::resources::TileSetIdMapper;
use crate::engine::{shared::TILE_MAP_COMPONENT_NAME, tree};

pub fn update_map(
    tile_set_id_mapper: Res<TileSetIdMapper>,
    owner: Res<Ref<Node, Shared>>,
    query: Query<(&Transform, &Tile, With<MapTile>), Changed<Tile>>,
) {
    for (transform, tile, _) in query.iter() {
        if let Some(tile_map) =
            unsafe { tree::get_tile_map(TILE_MAP_COMPONENT_NAME, &owner.assume_unique()) }
        {
            tile_map.set_cell(
                transform.position.x as i64,
                transform.position.y as i64,
                tile_set_id_mapper.get_tile_set_id(tile.id),
                false,
                false,
                false,
                Vector2::new(0., 0.),
            );
        }
    }
}
