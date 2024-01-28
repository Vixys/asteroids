use bevy::asset::AssetServer;
use bevy::prelude::{Bundle, default, Res, SpriteBundle};

use crate::movement::components::Movement;

use super::components::*;

#[derive(Bundle)]
pub struct AsteroidBundle {
    asteroid: Asteroid,
    sprite: SpriteBundle,
    pub movement: Movement,
}

impl AsteroidBundle {
    pub fn new(shape: AsteroidShape, asset_server: Res<AssetServer>) -> Self {
        Self {
            asteroid: Asteroid::new(&shape),
            sprite: SpriteBundle {
                texture: asset_server.load(shape),
                ..default()
            },
            movement: Movement { ..default() },
        }
    }
}