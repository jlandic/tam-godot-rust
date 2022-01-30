use bevy_ecs::prelude::*;

use crate::{
    data::geo::{pathfinding, Map},
    ecs::{components::*, resources::PlayerTransform},
};

pub fn ai_monster_dumb(
    map: Res<Map>,
    player_transform: Res<PlayerTransform>,
    mut query: Query<(&Monster, &Viewshed, &mut Path, &Transform)>,
) {
    for (_, viewshed, mut path, transform) in query.iter_mut() {
        if viewshed.visible_tiles.contains(&player_transform.position) {
            if player_transform
                .position
                .manhattan_distance(&transform.position)
                <= 1
            {
                path.points = vec![];
            } else {
                path.points =
                    pathfinding::search(&map, &transform.position, &player_transform.position);
            }
        }
    }
}
