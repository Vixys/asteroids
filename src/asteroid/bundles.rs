use bevy::asset::AssetServer;
use bevy::prelude::{default, Bundle, Res, SpriteBundle};

use crate::collider::components::*;
use crate::constants::{ASTEROID_COLLISION_LAYER, PLAYER_COLLISION_LAYER};
use crate::movement::components::Movement;
use crate::warp::components::Warp;

use super::components::*;

#[derive(Bundle)]
pub struct AsteroidBundle {
    asteroid: Asteroid,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    warp: Warp,
    pub collider: Collider,
}

impl AsteroidBundle {
    pub fn new(shape: AsteroidShape, asset_server: &Res<AssetServer>) -> Self {
        Self {
            asteroid: Asteroid::new(&shape),
            sprite: SpriteBundle {
                texture: asset_server.load(shape),
                ..default()
            },
            movement: Movement { ..default() },
            warp: Warp {},
            collider: Collider {
                shape: ColliderShape::Circle(24.0),
                collision_layer: ASTEROID_COLLISION_LAYER,
                collision_mask: PLAYER_COLLISION_LAYER,
            },
        }
    }
}
