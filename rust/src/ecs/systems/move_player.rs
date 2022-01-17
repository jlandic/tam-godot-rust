use crate::data::geo::{Map, Vec2};
use crate::ecs::components::{Player, Transform};
use crate::engine::resources::MovementInput;
use bevy_ecs::prelude::*;

pub fn move_player(
    movement: ResMut<MovementInput>,
    map: Res<Map>,
    mut transform: Query<&mut Transform, With<Player>>,
) {
    if movement.direction != Vec2::ZERO {
        let mut transform = transform.single_mut();
        let next_position = Vec2::new(
            transform.position.x + movement.direction.x,
            transform.position.y + movement.direction.y,
        );

        if !map.is_colliding(next_position) {
            transform.position = next_position;
        }
    }
}
