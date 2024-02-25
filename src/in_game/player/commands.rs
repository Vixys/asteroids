use crate::in_game::components::OnInGameScreen;
use bevy::{ecs::system::Command, prelude::*};

use super::bundles::PlayerBundle;
use super::constants::PLAYER_SHIP_ASSET_PATH;

#[derive(Debug)]
pub struct SpawnPlayer {
    position: Vec3,
}

impl Default for SpawnPlayer {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let mut player = PlayerBundle::default();

        player.sprite.transform.translation = self.position;
        player.sprite.texture = asset_server.load(PLAYER_SHIP_ASSET_PATH);

        world.spawn((player, OnInGameScreen));
    }
}
