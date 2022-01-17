use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::ecs::components::Tile;

pub fn update_tile(world: &mut World, owner: &Node) {
    let mut query = world.query::<(Entity, &Tile, Changed<Tile>)>();
    for (entity, tile, _) in query.iter(world) {
        if let Some(node) = owner.find_node(entity.id().to_string(), false, false) {
            let node = unsafe { node.assume_safe().cast::<Node2D>().unwrap() };

            unsafe {
                node.call("update_sprite", &[tile.id.to_variant()]);
            }
        }
    }
}
