use bevy::asset::AssetServer;
use bevy::prelude::{default, Bundle, Res, SpriteBundle};

use crate::collider::components::*;
use crate::constants::{ASTEROID_COLLISION_LAYER, PLAYER_COLLISION_LAYER};
use crate::movement::components::*;
use crate::warp::components::Warp;

use super::components::*;

#[derive(Bundle)]
pub struct AsteroidBundle {
    pub asteroid: Asteroid,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub warp: Warp,
    pub collider: Collider,
    pub angular_velocity: AngularVelocity,
}

impl Default for AsteroidBundle {
    fn default() -> Self {
        Self {
            asteroid: Asteroid::default(),
            sprite: SpriteBundle::default(),
            movement: Movement::default(),
            warp: Warp,
            collider: Collider {
                shape: ColliderShape::None,
                collision_layer: ASTEROID_COLLISION_LAYER,
                collision_mask: PLAYER_COLLISION_LAYER,
            },
            angular_velocity: AngularVelocity::default(),
        }
    }
}
