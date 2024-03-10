use std::f32::consts::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::common::movement::components::Movement;
use crate::helper::Rotate2D;
use crate::in_game::asteroid::constants::*;
use crate::in_game::collider::events::*;

use super::commands::SpawnAsteroid;
use super::components::*;
use super::resources::*;

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.insert_resource(AsteroidLineSpawner {
        points: vec![
            Vec2::new(
                -ASTEROID_PADDING_SPAWN_LOCATION - window.width() / 2.0,
                ASTEROID_PADDING_SPAWN_LOCATION + window.height() / 2.0,
            ),
            Vec2::new(
                ASTEROID_PADDING_SPAWN_LOCATION + window.width() / 2.0,
                ASTEROID_PADDING_SPAWN_LOCATION + window.height() / 2.0,
            ),
            Vec2::new(
                ASTEROID_PADDING_SPAWN_LOCATION + window.width() / 2.0,
                -ASTEROID_PADDING_SPAWN_LOCATION - window.height() / 2.0,
            ),
            Vec2::new(
                -ASTEROID_PADDING_SPAWN_LOCATION - window.width() / 2.0,
                -ASTEROID_PADDING_SPAWN_LOCATION - window.height() / 2.0,
            ),
        ],
    });

    for _ in 0..INITIAL_ASTEROID_COUNT {
        commands.add(SpawnAsteroid::random().with_size(AsteroidSize::Big));
    }
}

pub fn spawn_asteroid(mut commands: Commands, time: Res<Time>, mut timer: ResMut<AsteroidSpawner>) {
    if timer.timer.tick(time.delta()).just_finished() {
        commands.add(SpawnAsteroid::random());
    }
}

pub fn on_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(Entity, &Asteroid, &Transform, &Movement)>,
) {
    for event in collision_events.read() {
        let entity2 = query.get(event.entity2);

        if let Ok((entity, asteroid, transform, movement)) = entity2 {
            if let Some(size) = asteroid.size.shrink() {
                let new_asteroid = SpawnAsteroid::random()
                    .with_position(transform.translation)
                    .with_size(size)
                    .with_direction(movement.velocity.rotate_2d(FRAC_PI_2));
                let new_asteroid2 = SpawnAsteroid::random()
                    .with_position(transform.translation)
                    .with_size(size)
                    .with_direction(movement.velocity.rotate_2d(-FRAC_PI_2));
                commands.add(new_asteroid);
                commands.add(new_asteroid2);
            }
            commands.entity(entity).despawn();
        }
    }
    collision_events.clear();
}
