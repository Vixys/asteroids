use bevy::prelude::*;

const PLAYER_SHIP_ASSET_PATH: &str = "ship_B.png";

use crate::collider::components::*;
use crate::movement::components::Movement;
use crate::warp::components::Warp;

use super::components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
    movement: Movement,
    warp: Warp,
    collider: Collider,
}

impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> Self {
        Self {
            player: Player,
            sprite: SpriteBundle {
                texture: asset_server.load(PLAYER_SHIP_ASSET_PATH),
                ..default()
            },
            movement: Movement::default(),
            warp: Warp {},
            collider: Collider {
                shape: ColliderShape::Circle(16.0),
                collision_layer: 0b0000_0000_0000_0000_0000_0000_0000_0010,
                collision_mask: 0b0000_0000_0000_0000_0000_0000_0000_0001,
            },
        }
    }
}
