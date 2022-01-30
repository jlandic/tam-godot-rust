use bevy_ecs::event::Events;
use bevy_ecs::prelude::*;

use crate::ecs::resources::DebugConfig;
use crate::engine::events::{DebugAction, DebugConfigUpdate};

pub fn update_debug_config(
    mut config: ResMut<DebugConfig>,
    events: Res<Events<DebugConfigUpdate>>,
) {
    let mut reader = events.get_reader();

    for update in reader.iter(&events) {
        match update.action {
            DebugAction::ToggleDrawPath => {
                config.draw_path = !config.draw_path;
            }
            DebugAction::ToggleNoClip => config.no_clip = !config.no_clip,
        };
    }
}
