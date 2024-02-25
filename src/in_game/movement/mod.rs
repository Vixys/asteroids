pub mod components;
mod systems;

use systems::*;

use crate::game_state::GameState;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                movement_system.run_if(in_state(GameState::InGame)),
                angular_velocity_system.run_if(in_state(GameState::InGame)),
            ),
        );
    }
}
