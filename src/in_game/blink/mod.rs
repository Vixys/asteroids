use crate::in_game::blink::systems::*;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct BlinkPlugin;

impl Plugin for BlinkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, blink_system)
            .add_systems(PostUpdate, blink_removed);
    }
}
