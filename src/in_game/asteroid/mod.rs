use bevy::prelude::*;

mod bundles;
pub mod commands;
pub mod components;
mod constants;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::game_state::GameState;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(
                Update,
                (
                    /*spawn_asteroid.run_if(in_state(GameState::InGame)),*/
                    on_collision_system.run_if(in_state(GameState::InGame)),
                ),
            );
    }
}
