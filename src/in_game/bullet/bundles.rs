use crate::in_game::bullet::constants::*;
use bevy::prelude::*;

use crate::constants::*;
use crate::in_game::collider::components::*;
use crate::in_game::movement::components::Movement;

use super::components::Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub collider: Collider,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            bullet: Bullet {},
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::from(BULLET_SIZE),
                    ..default()
                },
                ..default()
            },
            movement: Movement {
                velocity: Vec2::ZERO,
                max_velocity: BULLET_SPEED,
            },
            collider: Collider {
                shape: ColliderShape::Circle(BULLET_SIZE.x),
                collision_layer: BULLET_COLLISION_LAYER,
                collision_mask: ASTEROID_COLLISION_LAYER,
            },
        }
    }
}
