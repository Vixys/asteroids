use bevy::{ecs::system::Command, prelude::*};
use rand::prelude::*;

use crate::movement::components::Movement;
use crate::{collider::components::*, movement::components::AngularVelocity};

use super::{
    bundles::AsteroidBundle,
    components::{Asteroid, AsteroidShape},
    constants::*,
    AsteroidLineSpawner,
};

#[derive(Debug)]
pub struct SpawnAsteroid {
    shape: AsteroidShape,
    size: f32,
    velocity_ratio: f32,
    angular_velocity: f32,
    direction: Option<Vec2>,
    position: Option<Vec3>,
}

impl From<Asteroid> for SpawnAsteroid {
    fn from(asteroid: Asteroid) -> Self {
        Self {
            shape: asteroid.shape,
            size: asteroid.size,
            velocity_ratio: 1.0,
            angular_velocity: 0.0,
            direction: None,
            position: None,
        }
    }
}

impl SpawnAsteroid {
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_direction(mut self, direction: Vec2) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let size_indice =
            rng.gen_range(*ASTEROID_SIZE_RANGE.start()..=*ASTEROID_SIZE_RANGE.end()) as f32;
        Self {
            shape: rng.gen(),
            size: 2.0_f32.powf(size_indice),
            velocity_ratio: rng.gen_range(ASTEROID_SPEED_RANGE),
            angular_velocity: ASTEROID_MAX_ANGULAR_VELOCITY / size_indice
                + rng.gen_range(ASTEROID_ANGULAR_VELOCITY_RANGE_VARIATION),
            direction: None,
            position: None,
        }
    }
}

impl Command for SpawnAsteroid {
    fn apply(self, world: &mut bevy::prelude::World) {
        let spawn_area = world.get_resource::<AsteroidLineSpawner>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let position = self
            .position
            .unwrap_or_else(|| spawn_area.random_point().extend(0.0));
        let direction = self
            .direction
            .unwrap_or_else(|| (Vec3::ZERO - position).truncate())
            .normalize();
        let asteroid = Asteroid {
            shape: self.shape,
            size: self.size,
        };

        let mut asteroid_bundle = AsteroidBundle {
            asteroid,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(self.size * 2.0)),
                    ..default()
                },
                texture: asset_server.load(asteroid.clone().get_asset_path()),
                ..default()
            },
            movement: Movement {
                velocity: direction * self.velocity_ratio * ASTEROID_MAX_SPEED,
                ..default()
            },
            angular_velocity: AngularVelocity {
                angular_velocity: self.angular_velocity,
                ..default()
            },
            ..default()
        };

        asteroid_bundle.sprite.transform.translation = position;
        asteroid_bundle.collider.shape = ColliderShape::Circle(self.size);

        println!("#### ASTEROID SPAWN ####");
        println!("{:?}", asteroid_bundle.movement);
        world.spawn(asteroid_bundle);
    }
}
