use crate::blink::components::Blink;
use bevy::prelude::*;

use crate::invincible::components::Invincible;
use crate::movement::components::Movement;
use crate::warp::components::Warp;

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
