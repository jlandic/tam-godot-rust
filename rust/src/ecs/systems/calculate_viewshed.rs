use crate::{data::geo::Map, ecs::components::*};
use bevy_ecs::prelude::*;

pub fn calculate_viewshed(
    map: Res<Map>,
    mut query: Query<(&mut Viewshed, &Transform), Changed<Transform>>,
) {
    for (mut viewshed, transform) in query.iter_mut() {
        let range = viewshed.range;
        viewshed.clear();
        viewshed.update_tiles(map.tiles_visible_from(transform.position, range));
    }
}
