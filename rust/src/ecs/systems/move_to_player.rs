use bevy_ecs::prelude::*;

use crate::ecs::components::*;

pub fn move_to_player(mut query: Query<(&Monster, &mut Path, &mut Transform)>) {
    for (_, mut path, mut transform) in query.iter_mut() {
        if let Some(next_point) = path.points.first() {
            transform.position = *next_point;
            path.points.remove(0);
        }
    }
}
