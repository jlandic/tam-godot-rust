use bevy_ecs::event::Events;
use bevy_ecs::prelude::*;

use crate::ecs::resources::{ConsoleMessages, DebugConfig};
use crate::engine::events::{DebugAction, DebugConfigUpdate};

pub fn update_debug_config(
    mut config: ResMut<DebugConfig>,
    events: Res<Events<DebugConfigUpdate>>,
    mut console_output: ResMut<ConsoleMessages>,
) {
    let mut reader = events.get_reader();

    for update in reader.iter(&events) {
        let message = match update.action {
            DebugAction::ToggleDrawPath => {
                config.draw_path = !config.draw_path;
                if config.draw_path {
                    "Draw path enabled."
                } else {
                    "Draw path disabled."
                }
            }
            DebugAction::ToggleNoClip => {
                config.no_clip = !config.no_clip;
                if config.no_clip {
                    "Player ignore collisions enabled."
                } else {
                    "Player ignore collisions disabled."
                }
            }
        };

        console_output.write_line(String::from(message))
    }
}
