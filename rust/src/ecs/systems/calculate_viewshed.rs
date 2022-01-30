use bevy_ecs::prelude::*;

use crate::{data::geo::Map, ecs::components::*};

type UpdatedViewshedOrTransform = Or<(Changed<Transform>, Changed<Viewshed>)>;

pub fn calculate_viewshed(
    mut map: ResMut<Map>,
    mut query: Query<(&mut Viewshed, &Transform, Option<&Player>), UpdatedViewshedOrTransform>,
) {
    for (mut viewshed, transform, player) in query.iter_mut() {
        let range = viewshed.range;
        viewshed.clear();

        let visible_tiles = if player.is_some() {
            map.assign_tiles_visible_from(transform.position, range)
        } else {
            map.tiles_visible_from(transform.position, range)
        };

        viewshed.update_tiles(visible_tiles);
    }
}
