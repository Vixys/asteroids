use bevy::prelude::*;

use crate::collider::components::*;
use crate::constants::*;
use crate::movement::components::Movement;

use super::components::Bullet;

const BULLET_ASSET_PATH: &str = "bullets/bullet.png";
const BULLET_SPEED: f32 = 500.0;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub collider: Collider,
}

impl BulletBundle {
    pub fn new(transform: &Transform, asset_server: &Res<AssetServer>) -> Self {
        Self {
            bullet: Bullet {},
            sprite: SpriteBundle {
                texture: asset_server.load(BULLET_ASSET_PATH),
                transform: *transform,
                ..default()
            },
            movement: Movement {
                velocity: transform.rotation.mul_vec3(Vec3::Y).truncate().normalize()
                    * BULLET_SPEED,
                max_velocity: BULLET_SPEED,
            },
            collider: Collider {
                shape: ColliderShape::Circle(32.0),
                collision_layer: BULLET_COLLISION_LAYER,
                collision_mask: ASTEROID_COLLISION_LAYER,
            },
        }
    }
}
