use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use std::f32::consts::*;
use std::ops::Range;

use crate::collider::components::Collider;
use crate::collider::components::ColliderShape;
use crate::collider::events::*;
use crate::movement::components::AngularVelocity;
use crate::movement::components::Movement;

use super::bundles::*;
use super::commands::SpawnAsteroid;
use super::components::*;
use super::resources::*;

const ASTEROID_MAX_SPEED: f32 = 100.0;
const ASTEROID_PADDING_SPAWN_LOCATION: f32 = 150.0;

const ASTEROID_SPEED_RANGE: Range<f32> = 0.65..1.1;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // commands.insert_resource(AsteroidSpawner {
    //     timer: Timer::from_seconds(5.0, TimerMode::Repeating),
    // });

    let window = window_query.get_single().unwrap();

    // let mut asteroid1 = AsteroidBundle::new(AsteroidShape::BigRound, &asset_server);
    // asteroid1.sprite.transform.translation = Vec3::new(-150.0, 150.0, 0.0);
    // asteroid1.movement.velocity = Vec2::new(0.0, -100.0);
    // commands.spawn(asteroid1);

    // let mut asteroid2 = AsteroidBundle::new(AsteroidShape::BigRound, &asset_server);
    // asteroid2.sprite.transform.translation = Vec3::new(-150.0, -150.0, 0.0);
    // asteroid2.movement.velocity = Vec2::new(-0.1, -0.1);
    // commands.spawn(asteroid2);

    // let mut asteroid3 = AsteroidBundle::new(AsteroidShape::BigRound, &asset_server);
    // asteroid3.sprite.transform.translation = Vec3::new(150.0, -150.0, 0.0);
    // asteroid3.movement.velocity = Vec2::new(0.1, -0.1);
    // commands.spawn(asteroid3);

    // let mut asteroid4 = AsteroidBundle::new(AsteroidShape::BigRound, &asset_server);
    // asteroid4.sprite.transform.translation = Vec3::new(150.0, 150.0, 0.0);
    // asteroid4.movement.velocity = Vec2::new(0.1, 0.1);
    // commands.spawn(asteroid4);

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

    for _ in 0..10 {
        commands.add(SpawnAsteroid::random());
    }

    // TOP
    // commands.spawn(AsteroidSpawnArea {
    //     from: Vec2::new(
    //         0.0 - window.width() / 2.0,
    //         -ASTEROID_PADDING_SPAWN_LOCATION - window.height() / 2.0,
    //     ),
    //     to: Vec2::new(
    //         window.width() / 2.0,
    //         -ASTEROID_PADDING_SPAWN_LOCATION - window.height() / 2.0,
    //     ),
    //     spawn_angle_range: (5.0 * FRAC_PI_4)..(7.0 * FRAC_PI_4),
    // });
    // // BOTTOM
    // commands.spawn(AsteroidSpawnArea {
    //     from: Vec2::new(
    //         0.0 - window.width() / 2.0,
    //         ASTEROID_PADDING_SPAWN_LOCATION + window.height() / 2.0,
    //     ),
    //     to: Vec2::new(
    //         window.width() / 2.0,
    //         ASTEROID_PADDING_SPAWN_LOCATION + window.height() / 2.0,
    //     ),
    //     spawn_angle_range: (1.0 * FRAC_PI_4)..(3.0 * FRAC_PI_4),
    // });
    // // LEFT
    // commands.spawn(AsteroidSpawnArea {
    //     from: Vec2::new(
    //         -ASTEROID_PADDING_SPAWN_LOCATION - window.width() / 2.0,
    //         0.0 - window.height() / 2.0,
    //     ),
    //     to: Vec2::new(
    //         -ASTEROID_PADDING_SPAWN_LOCATION - window.width() / 2.0,
    //         window.height() / 2.0,
    //     ),
    //     spawn_angle_range: (-1.0 * FRAC_PI_4)..(1.0 * FRAC_PI_4),
    // });
    // // RIGHT
    // commands.spawn(AsteroidSpawnArea {
    //     from: Vec2::new(
    //         ASTEROID_PADDING_SPAWN_LOCATION + window.width() / 2.0,
    //         0.0 - window.height() / 2.0,
    //     ),
    //     to: Vec2::new(
    //         ASTEROID_PADDING_SPAWN_LOCATION + window.width() / 2.0,
    //         window.height() / 2.0,
    //     ),
    //     spawn_angle_range: (3.0 * FRAC_PI_4)..(5.0 * FRAC_PI_4),
    // });
}

pub fn asteroid_spawned_system(
    mut query: Query<
        (
            &Asteroid,
            &mut Handle<Image>,
            &mut Collider,
            &mut AngularVelocity,
            &mut Sprite,
        ),
        Added<Asteroid>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (asteroid, mut texture, mut collider, mut angular_velocity, mut sprite) in query.iter_mut()
    {
        collider.shape = ColliderShape::Circle(asteroid.size);
        angular_velocity.angular_velocity = asteroid.get_angular_velocity();
        *texture = asset_server.load(asteroid.get_asset_path());
    }
}

pub fn spawn_asteroid(
    mut commands: Commands,
    spawn_area_query: Query<&AsteroidSpawnArea>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawner>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        if let Some(area) = spawn_area_query.iter().choose(&mut rng) {
            println!("#### ASTEROID SPAWN ####");
            print!("area: {:?}", area);
            let mut asteroid = AsteroidBundle {
                asteroid: Asteroid {
                    shape: rng.gen(),
                    ..default()
                },
                sprite: SpriteBundle::default(),
                movement: Movement {
                    velocity: Quat::from_rotation_z(rng.gen_range(area.spawn_angle_range.clone()))
                        .mul_vec3(Vec3::X)
                        .truncate()
                        .normalize()
                        * rng.gen_range(ASTEROID_SPEED_RANGE.clone())
                        * ASTEROID_MAX_SPEED,
                    ..default()
                },
                ..default()
            };
            // asteroid.sprite.transform.translation =
            //     area.from.lerp(area.to, rng.gen::<f32>()).extend(0.0);
            asteroid.sprite.transform.translation = Vec3::new(-150.0, 150.0, 0.0);
            println!("position: {:?}", asteroid.sprite.transform.translation);
            println!("velocity: {:?}", asteroid.movement.velocity);
            commands.spawn(asteroid);
        } else {
            println!("No spawn area found!");
        }
    }
}

pub fn on_collistion_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(Entity, &Transform, &Movement), With<Asteroid>>,
) {
    for event in collision_events.read() {
        let entity2 = query.get(event.entity2);

        if let Ok((entity, transform, movement)) = entity2 {
            let new_asteroid = create_asteroid(transform, &movement.velocity, FRAC_PI_2);
            let new_asteroid2 = create_asteroid(transform, &movement.velocity, -FRAC_PI_2);
            println!("#### ASTEROID SPAWN ####");
            println!("position: {:?}", new_asteroid.sprite.transform.translation);
            println!("velocity: {:?}", new_asteroid.movement.velocity);
            println!("#### ASTEROID SPAWN ####");
            println!("position: {:?}", new_asteroid2.sprite.transform.translation);
            println!("velocity: {:?}", new_asteroid2.movement.velocity);
            // commands.spawn(new_asteroid);
            // commands.spawn(new_asteroid2);
            println!("Asteroid collision: ");
            commands.entity(entity).despawn();
        }
    }
}

fn create_asteroid(transform: &Transform, velocity: &Vec2, angle: f32) -> AsteroidBundle {
    let mut rng = thread_rng();
    let mut asteroid = AsteroidBundle::default();

    asteroid.sprite.transform = *transform;
    asteroid.movement.velocity =
        Quat::from_rotation_z(angle + rng.gen_range(-FRAC_PI_6..=FRAC_PI_6))
            .mul_vec3(velocity.extend(0.0))
            .truncate()
            .normalize()
            * rng.gen_range(ASTEROID_SPEED_RANGE.clone())
            * ASTEROID_MAX_SPEED;

    asteroid
}
