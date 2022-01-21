use bevy_ecs::prelude::*;

use crate::data::geo::Vec2;

#[derive(Component, Default)]
pub struct Viewshed {
    pub visible_tiles: Vec<Vec2>,
    pub range: u32,
}

impl Viewshed {
    pub fn new(range: u32) -> Self {
        Self {
            range,
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.visible_tiles.clear();
    }

    pub fn update_tiles(&mut self, visible_tiles: Vec<Vec2>) {
        self.visible_tiles = visible_tiles;
    }
}
