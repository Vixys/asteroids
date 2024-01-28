use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct AsteroidSpawner {
    pub timer: Timer,
}
