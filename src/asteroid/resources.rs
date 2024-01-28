use bevy::prelude::{Resource, Timer, Vec2};

#[derive(Resource)]
pub struct AsteroidSpawner {
    pub timer: Timer,
}
