use bevy::prelude::*;

mod components;
mod systems;
mod resources;
mod bundles;

use resources::*;
use systems::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawner {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        })
            .add_systems(Update, (spawn_asteroid, asteroid_rotate_system));
    }
}