use bevy::prelude::*;

const PLAYER_SHIP_ASSET_PATH: &str = "ship_B.png";

use crate::invincible::components::Invincible;
use crate::movement::components::Movement;
use crate::warp::components::Warp;

use super::components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
    movement: Movement,
    warp: Warp,
    invicibility: Invincible,
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
            invicibility: Invincible::default(),
        }
    }
}
