use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::config::WorldConfig;
use crate::data::geo::Vec2;
use crate::ecs::components::Transform;
use crate::engine::Owner;

pub fn update_position(
    owner: Res<Owner>,
    world_config: Res<WorldConfig>,
    query: Query<(Entity, &Transform), Changed<Transform>>,
) {
    for (entity, transform) in query.iter() {
        if let Some(node) = unsafe {
            owner
                .assume_unique()
                .find_node(entity.id().to_string(), false, false)
        } {
            let node = unsafe { node.assume_safe().cast::<Node2D>().unwrap() };

            let to_position: Vector2 = Vec2::new(
                transform.position.x * world_config.tile_size as i32
                    + (world_config.tile_size as i32 / 2),
                transform.position.y * world_config.tile_size as i32
                    + (world_config.tile_size as i32 / 2),
            )
            .into();

            unsafe {
                node.call("move_tween", &[to_position.to_variant()]);
            }
        }
    }
}
