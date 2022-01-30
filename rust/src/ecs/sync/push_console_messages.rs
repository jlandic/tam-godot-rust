use bevy_ecs::prelude::*;
use gdnative::prelude::*;

use crate::ecs::resources::ConsoleMessages;
use crate::engine::Owner;

pub fn push_console_messages(owner: Res<Owner>, mut messages: ResMut<ConsoleMessages>) {
    while let Some(message) = messages.pop() {
        unsafe {
            owner
                .assume_safe()
                .get_parent()
                .unwrap()
                .assume_safe()
                .call("console_print", &[message.to_variant()])
        };
    }
}
