use bevy_ecs::prelude::*;
use crate::{ecs::components::*, data::geo::Map};

pub fn calculate_viewshed(
    map: Res<Map>,
    mut query: Query<(&mut Viewshed, &Transform)>,
) {
    for (mut viewshed, transform) in query.iter_mut() {
        let range = viewshed.range;

        viewshed.clear();
        viewshed.update_tiles(map.square(transform.position, range));
    }
}
