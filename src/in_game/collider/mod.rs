use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use events::*;
use systems::*;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(FixedUpdate, check_collision);
    }
}
