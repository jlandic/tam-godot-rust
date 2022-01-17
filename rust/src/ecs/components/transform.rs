use crate::data::geo::Vec2;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Transform {
    pub position: Vec2,
}

impl Transform {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}
