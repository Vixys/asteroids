use super::components::UiLive;
use crate::in_game::player::constants::PLAYER_SHIP_ASSET_PATH;
use bevy::ecs::system::Command;
use bevy::prelude::*;

impl Command for UiLive {
    fn apply(self, world: &mut World) {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let ui_live = ImageBundle {
            transform: Transform {
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            image: UiImage {
                texture: asset_server.load(PLAYER_SHIP_ASSET_PATH),
                ..Default::default()
            },
            ..Default::default()
        };
        world.spawn((ui_live, UiLive));
    }
}
