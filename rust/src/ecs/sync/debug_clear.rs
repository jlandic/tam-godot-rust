use bevy_ecs::prelude::*;

use crate::data::geo::Map;
use crate::engine::shared::DEBUG_TILE_MAP_COMPONENT_NAME;
use crate::engine::{tree, Owner};

#[allow(dead_code)]
pub fn debug_clear(map: Res<Map>, owner: Res<Owner>) {
    unsafe {
        tree::clear_map(
            DEBUG_TILE_MAP_COMPONENT_NAME,
            &owner.assume_unique(),
            true,
            map.size(),
        );
    }
}
