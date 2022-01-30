use bevy_ecs::prelude::*;

use crate::data::geo::Vec2;

#[derive(Component)]
pub struct Transform {
    pub position: Vec2,
}

impl Transform {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}
