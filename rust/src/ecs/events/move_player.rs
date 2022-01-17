use hecs::{Entity, World};

use crate::data::geo::{Map, Vec2};
use crate::ecs::components::{Player, Position};
use crate::ecs::messages::Moved;

pub fn move_player(world: &mut World, direction: Vec2, map: &Map) {
    let mut to_move: Vec<Entity> = Vec::new();

    for (entity, (_player, position)) in world.query_mut::<(&Player, &mut Position)>() {
        let next_position = Position::new(position.x + direction.x, position.y + direction.y);

        if !map.is_colliding(next_position) {
            *position = next_position;

            to_move.push(entity);
        }
    }

    for entity in to_move {
        let _ = world.insert(entity, (Moved,));
    }
}
