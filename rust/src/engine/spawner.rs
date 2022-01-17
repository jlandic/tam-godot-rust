use gdnative::prelude::*;
use hecs::{DynamicBundle, World};

pub struct SceneSpawner;

impl SceneSpawner {
    pub fn spawn(
        world: &mut World,
        base_scene: &Ref<PackedScene>,
        owner: &Node,
        components: impl DynamicBundle,
    ) {
        let id = world.spawn(components).id();
        let scene = unsafe {
            base_scene
                .assume_safe()
                .instance(0)
                .unwrap()
                .assume_unique()
                .cast::<Node2D>()
                .unwrap()
        };

        scene.set_name(id.to_string());
        owner.add_child(scene, true);
    }
}
