use std::ops::Range;

use bevy::{asset::AssetPath, prelude::*};

use crate::movement::{Movement, MovementPlugin};

#[derive(Resource)]
struct AsteroidSpanwer {
    timer: Timer,
}

#[derive(Clone, Copy)]
enum AsteroidShape {
    SmallRound,
    BigRound,
    SmallSquare,
    BigSquare,
}

impl<'a> Into<AssetPath<'a>> for AsteroidShape {
    fn into(self) -> AssetPath<'a> {
        match self {
            AsteroidShape::SmallRound => "asteroids/asteroid_small.png".into(),
            AsteroidShape::BigRound => "asteroids/asteroid_big.png".into(),
            AsteroidShape::SmallSquare => "asteroids/asteroid_square_small.png".into(),
            AsteroidShape::BigSquare => "asteroids/asteroid_square_big.png".into(),
        }
    }
}

impl AsteroidShape {
    fn get_random_rotation_speed(&self) -> Range<f32> {
        match self {
            AsteroidShape::SmallRound => 0.75..1.0,
            AsteroidShape::BigRound => 0.3..0.65,
            AsteroidShape::SmallSquare => 0.75..1.0,
            AsteroidShape::BigSquare => 0.3..0.65,
        }
    }
}

#[derive(Component)]
struct Asteroid {
    shape: AsteroidShape,
    rotation_speed: f32,
}

impl Asteroid {
    fn new(shape: &AsteroidShape) -> Self {
        Self {
            shape: shape.clone(),
            rotation_speed: shape.get_random_rotation_speed().start,
        }
    }
}

#[derive(Bundle)]
struct AsteroidBundle {
    asteroid: Asteroid,
    sprite: SpriteBundle,
    movement: Movement,
}

impl AsteroidBundle {
    fn new(shape: AsteroidShape, asset_server: Res<AssetServer>) -> Self {
        Self {
            asteroid: Asteroid::new(&shape),
            sprite: SpriteBundle {
                texture: asset_server.load(shape),
                ..default()
            },
            movement: Movement { ..default() },
        }
    }
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpanwer {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        })
        .add_systems(Update, (spawn_asteroid, asteroid_rotate_system));
    }
}

fn asteroid_rotate_system(time: Res<Time>, mut query: Query<(&Asteroid, &mut Transform)>) {
    for (asteroid, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            asteroid.rotation_speed * time.delta_seconds(),
        ));
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpanwer>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        let mut asteroid = AsteroidBundle::new(AsteroidShape::BigRound, asset_server);
        asteroid.movement.velocity = Vec2::new(50.0, -100.0);
        commands.spawn(asteroid);
    }
}
