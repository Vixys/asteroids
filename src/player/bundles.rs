use bevy::prelude::*;

const PLAYER_SHIP_ASSET_PATH: &str = "ship_B.png";

use crate::movement::components::Movement;

use super::components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
    movement: Movement,
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
        }
    }
}