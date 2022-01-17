use gdnative::prelude::*;
use hecs::{Entity, World};

use crate::ecs::components::Tile;
use crate::ecs::messages::TileChanged;

pub fn update_tile(world: &mut World, owner: &Node) {
    let mut to_remove: Vec<Entity> = Vec::new();
    for (entity, (_type_changed, tile)) in world.query_mut::<(&TileChanged, &Tile)>() {
        if let Some(node) = owner.find_node(entity.id().to_string(), false, false) {
            let node = unsafe { node.assume_safe().cast::<Node2D>().unwrap() };

            unsafe {
                node.call("update_sprite", &[tile.id.to_variant()]);
            }
        }

        to_remove.push(entity);
    }

    for entity in to_remove {
        let _ = world.remove_one::<TileChanged>(entity);
    }
}
