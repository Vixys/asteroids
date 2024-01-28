use bevy::prelude::*;

use super::components::*;
use super::resources::*;
use super::bundles::*;
use crate::warp::components::Warp;

pub fn asteroid_rotate_system(time: Res<Time>, mut query: Query<(&Asteroid, &mut Transform)>) {
    for (asteroid, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            asteroid.rotation_speed * time.delta_seconds(),
        ));
    }
}

pub fn spawn_asteroid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawner>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        let mut asteroid = AsteroidBundle::new(AsteroidShape::BigRound, asset_server);
        asteroid.movement.velocity = Vec2::new(50.0, -100.0);
        commands.spawn((asteroid, Warp));
    }
}
