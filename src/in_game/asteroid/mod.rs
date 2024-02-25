use bevy::prelude::*;

mod bundles;
mod commands;
mod components;
mod constants;
mod resources;
mod systems;

use crate::game_state::GameState;
use resources::*;
use systems::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawner {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        })
        .add_systems(OnEnter(GameState::InGame), setup)
        .add_systems(
            Update,
            (
                spawn_asteroid.run_if(in_state(GameState::InGame)),
                on_collision_system.run_if(in_state(GameState::InGame)),
            ),
        );
    }
}
