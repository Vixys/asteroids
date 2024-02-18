use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::*;

use crate::asteroid::constants::*;
use crate::collider::events::*;
use crate::movement::components::Movement;

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
        commands.add(SpawnAsteroid::random().with_size(128.0));
    }
}

pub fn spawn_asteroid(mut commands: Commands, time: Res<Time>, mut timer: ResMut<AsteroidSpawner>) {
    if timer.timer.tick(time.delta()).just_finished() {
        commands.add(SpawnAsteroid::random());
    }
}

pub fn on_collistion_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(Entity, &Asteroid, &Transform, &Movement)>,
) {
    for event in collision_events.read() {
        let entity2 = query.get(event.entity2);

        if let Ok((entity, asteroid, transform, movement)) = entity2 {
            if asteroid.size > ASTEROID_SMALLEST_SIZE {
                let new_asteroid = SpawnAsteroid::random()
                    .with_position(transform.translation)
                    .with_size(asteroid.size / 2.0)
                    .with_direction(
                        Quat::from_rotation_z(FRAC_PI_2)
                            .mul_vec3(movement.velocity.extend(0.0))
                            .truncate()
                            .normalize(),
                    );
                let new_asteroid2 = SpawnAsteroid::random()
                    .with_position(transform.translation)
                    .with_size(asteroid.size / 2.0)
                    .with_direction(
                        Quat::from_rotation_z(-FRAC_PI_2)
                            .mul_vec3(movement.velocity.extend(0.0))
                            .truncate()
                            .normalize(),
                    );
                println!("#### ASTEROID SPAWN ####");
                println!("ASTEROID: {:?}", new_asteroid);
                println!("#### ASTEROID SPAWN ####");
                println!("ASTEROID: {:?}", new_asteroid2);
                commands.add(new_asteroid);
                commands.add(new_asteroid2);
            }
            println!("Asteroid collision: ");
            commands.entity(entity).despawn();
        }
    }
}
