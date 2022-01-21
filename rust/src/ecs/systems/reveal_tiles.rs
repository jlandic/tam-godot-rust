use crate::{data::geo::Map, ecs::components::*};
use bevy_ecs::prelude::*;

pub fn reveal_tiles(
    mut map: ResMut<Map>,
    query: Query<(&Viewshed, With<Player>), Changed<Viewshed>>,
) {
    for (viewshed, _) in query.iter() {
        for tile in &viewshed.visible_tiles {
            map.reveal_tile(*tile);
        }
    }
}
