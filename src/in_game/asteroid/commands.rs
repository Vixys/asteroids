use bevy::{ecs::system::Command, prelude::*};
use rand::prelude::*;

use crate::common::movement::components::*;
use crate::helper::*;
use crate::in_game::collider::components::*;
use crate::in_game::components::OnInGameScreen;

use super::components::AsteroidSize;
use super::{
    bundles::AsteroidBundle,
    components::{Asteroid, AsteroidShape},
    constants::*,
    AsteroidLineSpawner,
};

#[derive(Debug)]
pub struct SpawnAsteroid {
    shape: AsteroidShape,
    size: AsteroidSize,
    radius: f32,
    velocity_ratio: f32,
    max_speed: f32,
    angular_velocity: f32,
    direction: Option<Vec2>,
    direction_noise: f32,
    position: Option<Vec3>,
}

impl SpawnAsteroid {
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_max_speed(mut self, max_speed: f32) -> Self {
        self.max_speed = max_speed;
        self
    }

    pub fn with_size(mut self, size: AsteroidSize) -> Self {
        let mut rng = thread_rng();
        self.size = size;
        self.radius = rng.gen_range(size.get_radius_range());
        self
    }

    pub fn with_direction(mut self, direction: Vec2) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let size: AsteroidSize = rng.gen();
        Self {
            shape: rng.gen(),
            size,
            velocity_ratio: rng.gen_range(size.get_speed_range()),
            angular_velocity: ASTEROID_MAX_ANGULAR_VELOCITY * rng.gen_range(size.get_speed_range()),
            radius: rng.gen_range(size.get_radius_range()),
            max_speed: ASTEROID_MAX_SPEED,
            direction: None,
            direction_noise: rng.gen_range(ASTEROID_DIRECTION_NOISE_RANGE),
            position: None,
        }
    }
}

impl Command for SpawnAsteroid {
    fn apply(self, world: &mut World) {
        let spawn_area = world.get_resource::<AsteroidLineSpawner>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let position = self.position.unwrap_or_else(|| spawn_area.random_point().extend(0.0));
        let direction = self.direction.unwrap_or_else(|| (Vec3::ZERO - position).truncate()).rotate_2d(self.direction_noise);
        let asteroid = Asteroid {
            shape: self.shape,
            size: self.size,
        };

        let mut asteroid_bundle = AsteroidBundle {
            asteroid,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(self.radius * 2.0)),
                    ..default()
                },
                texture: asset_server.load(asteroid.clone().get_asset_path()),
                ..default()
            },
            movement: Movement {
                velocity: direction * self.velocity_ratio * self.max_speed,
                ..default()
            },
            angular_velocity: AngularVelocity {
                angular_velocity: self.angular_velocity,
                ..default()
            },
            ..default()
        };

        asteroid_bundle.sprite.transform.translation = position;
        asteroid_bundle.collider.shape = ColliderShape::Circle(self.radius);

        info!("Asteroid Spawn: {:?}", self);
        world.spawn((asteroid_bundle, OnInGameScreen));
    }
}
