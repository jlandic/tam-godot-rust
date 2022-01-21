use bevy_ecs::prelude::*;

use crate::data::geo::Map;
use crate::ecs::components::*;
use crate::engine::{tree, Owner};

type NotStatic = Without<Static>;
type NotPlayer = Without<Player>;

pub fn hide_entities_outside_viewshed(
    owner: Res<Owner>,
    map: Res<Map>,
    query: Query<((Entity, &Transform, &Tile), NotStatic, NotPlayer)>,
) {
    for ((entity, transform, _), _, _) in query.iter() {
        let node = unsafe {
            tree::get_entity_node(&owner.assume_unique(), &entity.id().to_string()).unwrap()
        };
        if map.visible.contains(&transform.position) {
            node.set_visible(true);
        } else {
            node.set_visible(false);
        }
    }
}
