use bevy::prelude::*;

use std::f32::consts::{FRAC_PI_2, PI};

use crate::warp::components::Warp;
use crate::movement::components::Movement;

use super::bundles::PlayerBundle;
use super::components::*;


const PLAYER_ROTATION_SPEED: f32 = PI;
const PLAYER_SPEED: f32 = 300.0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((PlayerBundle::new(asset_server), Warp::default()));
}

pub fn player_input(
    mut query: Query<(&mut Movement, &mut Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut movement, mut player_transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            player_transform.rotate_z(PLAYER_ROTATION_SPEED * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player_transform.rotate_z(-PLAYER_ROTATION_SPEED * time.delta_seconds());
        }

        if keyboard_input.pressed(KeyCode::R) {
            player_transform.rotation = Quat::from_rotation_z(0.0);
            player_transform.translation = Vec3::ZERO;
            movement.velocity = Vec2::ZERO;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            let direction =
                Vec2::from_angle(player_transform.rotation.to_scaled_axis().z + FRAC_PI_2)
                    .normalize();
            movement.velocity += direction * PLAYER_SPEED * time.delta_seconds();
        }
    }
}