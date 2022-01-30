use gdnative::prelude::*;

pub use spawner::SceneSpawner;

mod spawner;

pub mod events;
pub mod resources;
pub mod shared;
pub mod tree;

pub type Owner = Ref<Node, Shared>;
