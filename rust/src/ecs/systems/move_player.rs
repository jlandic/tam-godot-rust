use crate::data::geo::{Map, Vec2};
use crate::ecs::components::*;
use crate::engine::events::MovementInput;
use bevy_ecs::event::Events;
use bevy_ecs::prelude::*;

pub fn move_player(
    movements: Res<Events<MovementInput>>,
    map: Res<Map>,
    mut transform: Query<&mut Transform, With<Player>>,
) {
    let mut reader = movements.get_reader();

    for movement in reader.iter(&movements) {
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
}
