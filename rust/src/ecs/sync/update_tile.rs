use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::{ecs::components::Tile, engine::Owner};

pub fn update_tile(owner: Res<Owner>, query: Query<(Entity, &Tile), Changed<Tile>>) {
    for (entity, tile) in query.iter() {
        if let Some(node) =
            unsafe { owner.assume_unique() }.find_node(entity.id().to_string(), false, false)
        {
            let node = unsafe { node.assume_safe().cast::<Node2D>().unwrap() };

            unsafe {
                node.call("update_sprite", &[tile.id.id().to_variant()]);
            }
        }
    }
}
