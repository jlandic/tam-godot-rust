use bevy_ecs::prelude::*;
use gdnative::godot_print;

use crate::ecs::{components::*, resources::PlayerTransform};

pub fn ai_monster_dumb(
    player_transform: Res<PlayerTransform>,
    query: Query<(&Monster, &Viewshed, Option<&Identity>)>,
) {
    for (_, viewshed, identity) in query.iter() {
        if viewshed.visible_tiles.contains(&player_transform.position) {
            if let Some(identity) = identity {
                godot_print!("{} shouts: I see you!", identity.name);
            } else {
                godot_print!("Player is visible!");
            }
        }
    }
}
