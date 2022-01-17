use gdnative::prelude::{godot_init, InitHandle};

mod config;
mod data;
mod ecs;
mod engine;
mod generators;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<ecs::Ecs>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
