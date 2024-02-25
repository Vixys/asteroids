use bevy::prelude::*;

mod bundles;
mod commands;
mod components;
mod constants;
mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawner {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_asteroid, on_collision_system));
    }
}
