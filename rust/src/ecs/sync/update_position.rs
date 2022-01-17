use gdnative::prelude::*;
use hecs::{Entity, World};

use crate::config::WorldConfig;
use crate::data::geo::Vec2;
use crate::ecs::components::Position;
use crate::ecs::messages::Moved;

pub fn update_position(world: &mut World, owner: &Node, world_config: &WorldConfig) {
    let to_remove: Vec<Entity> = Vec::new();

    for (entity, (_moved, position)) in world.query_mut::<(&Moved, &Position)>() {
        if let Some(node) = owner.find_node(entity.id().to_string(), false, false) {
            let node = unsafe { node.assume_safe().cast::<Node2D>().unwrap() };
            let to_position: Vector2 = Vec2::new(
                position.x * world_config.tile_size as i32 + (world_config.tile_size as i32 / 2),
                position.y * world_config.tile_size as i32 + (world_config.tile_size as i32 / 2),
            )
            .into();

            unsafe {
                node.call("move_tween", &[to_position.to_variant()]);
            }
        }
    }

    for entity in to_remove {
        let _ = world.remove_one::<Moved>(entity);
    }
}
