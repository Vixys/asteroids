use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::distributions::Standard;
use rand::prelude::*;
use std::f32::consts::*;
use std::ops::Range;

use crate::warp::components::Warp;

use super::bundles::*;
use super::components::*;
use super::resources::*;

const ASTEROID_MAX_SPEED: f32 = 100.0;
const ASTEROID_PADDING_SPAWN_LOCATION: f32 = 150.0;

static ASTEROID_SPEED_RANGE: Range<f32> = 0.65..1.1;

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    commands.insert_resource(AsteroidSpawner {
        timer: Timer::from_seconds(5.0, TimerMode::Repeating),
    });

    let window = window_query.get_single().unwrap();

    // TOP
    commands.spawn(AsteroidSpawnArea {
        from: Vec2::new(
            0.0 - window.width() / 2.0,
            -ASTEROID_PADDING_SPAWN_LOCATION - window.height() / 2.0,
        ),
        to: Vec2::new(
            window.width() / 2.0,
            -ASTEROID_PADDING_SPAWN_LOCATION - window.height() / 2.0,
        ),
        spawn_angle_range: (5.0 * FRAC_PI_4)..(7.0 * FRAC_PI_4),
    });
    // BOTTOM
    commands.spawn(AsteroidSpawnArea {
        from: Vec2::new(
            0.0 - window.width() / 2.0,
            ASTEROID_PADDING_SPAWN_LOCATION + window.height() / 2.0,
        ),
        to: Vec2::new(
            window.width() / 2.0,
            ASTEROID_PADDING_SPAWN_LOCATION + window.height() / 2.0,
        ),
        spawn_angle_range: (1.0 * FRAC_PI_4)..(3.0 * FRAC_PI_4),
    });
    // LEFT
    commands.spawn(AsteroidSpawnArea {
        from: Vec2::new(
            -ASTEROID_PADDING_SPAWN_LOCATION - window.width() / 2.0,
            0.0 - window.height() / 2.0,
        ),
        to: Vec2::new(
            -ASTEROID_PADDING_SPAWN_LOCATION - window.width() / 2.0,
            window.height() / 2.0,
        ),
        spawn_angle_range: (-1.0 * FRAC_PI_4)..(1.0 * FRAC_PI_4),
    });
    // RIGHT
    commands.spawn(AsteroidSpawnArea {
        from: Vec2::new(
            ASTEROID_PADDING_SPAWN_LOCATION + window.width() / 2.0,
            0.0 - window.height() / 2.0,
        ),
        to: Vec2::new(
            ASTEROID_PADDING_SPAWN_LOCATION + window.width() / 2.0,
            window.height() / 2.0,
        ),
        spawn_angle_range: (3.0 * FRAC_PI_4)..(5.0 * FRAC_PI_4),
    });
}

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
    spawn_area_query: Query<&AsteroidSpawnArea>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawner>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        if let Some(area) = spawn_area_query.iter().choose(&mut rng) {
            println!("#### ASTEROID SPAWN ####");
            print!("area: {:?}", area);
            let mut asteroid = AsteroidBundle::new(AsteroidShape::BigRound, asset_server);
            asteroid.movement.velocity =
                Quat::from_rotation_z(rng.gen_range(area.spawn_angle_range.clone()))
                    .mul_vec3(Vec3::X)
                    .truncate()
                    .normalize()
                    * rng.gen_range(ASTEROID_SPEED_RANGE.clone())
                    * ASTEROID_MAX_SPEED;
            asteroid.sprite.transform.translation =
                area.from.lerp(area.to, rng.gen::<f32>()).extend(0.0);
            println!("position: {:?}", asteroid.sprite.transform.translation);
            println!("velocity: {:?}", asteroid.movement.velocity);
            commands.spawn((asteroid, Warp));
        } else {
            println!("No spawn area found!");
        }
    }
}
