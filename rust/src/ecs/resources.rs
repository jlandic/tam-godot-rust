use crate::data::geo::Vec2;

pub struct PlayerTransform {
    pub position: Vec2,
}

impl PlayerTransform {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

#[derive(Debug, Default)]
pub struct DebugConfig {
    pub draw_path: bool,
    pub no_clip: bool,
}
