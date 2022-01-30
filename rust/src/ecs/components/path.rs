use bevy_ecs::prelude::*;

use crate::data::geo::Vec2;

#[derive(Component, Default)]
pub struct Path {
    pub points: Vec<Vec2>,
}
