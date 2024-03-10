use crate::in_game::blink::components::Blink;
use bevy::prelude::*;

use crate::common::movement::components::Movement;
use crate::common::warp::components::Warp;
use crate::in_game::invincible::components::Invincible;
use crate::in_game::player::constants::PLAYER_START_LIVES;

use super::components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite: SpriteBundle,
    pub movement: Movement,
    pub warp: Warp,
    pub invincible: Invincible,
    pub blink: Blink,
    pub lives: PlayerLives,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player {
                fire_cooldown: Timer::from_seconds(0.2, TimerMode::Once),
            },
            sprite: SpriteBundle::default(),
            movement: Movement::default(),
            warp: Warp {},
            invincible: Invincible::default(),
            blink: Blink::default(),
            lives: PlayerLives(PLAYER_START_LIVES),
        }
    }
}
