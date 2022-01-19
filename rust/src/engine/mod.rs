use gdnative::prelude::*;

mod spawner;

pub mod events;
pub mod resources;
pub mod shared;
pub mod tree;

pub use spawner::SceneSpawner;

pub type Owner = Ref<Node, Shared>;
