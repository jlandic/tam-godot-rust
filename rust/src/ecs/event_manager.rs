use bevy_ecs::{event::Events, prelude::*};

pub fn purge_events<T: 'static + Send + Sync>(mut events: ResMut<Events<T>>) {
    events.clear();
}
