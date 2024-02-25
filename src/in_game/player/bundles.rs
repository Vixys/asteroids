use crate::in_game::blink::components::Blink;
use bevy::prelude::*;

use crate::in_game::invincible::components::Invincible;
use crate::in_game::movement::components::Movement;
use crate::in_game::warp::components::Warp;

use super::components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub warp: Warp,
    pub invincible: Invincible,
    pub blink: Blink,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite: SpriteBundle::default(),
            movement: Movement::default(),
            warp: Warp {},
            invincible: Invincible::default(),
            blink: Blink::default(),
        }
    }
}
