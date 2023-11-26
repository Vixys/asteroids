use std::f32::consts::{FRAC_PI_2, PI};

use crate::movement::*;
use bevy::prelude::*;

const PLAYER_SHIP_ASSET_PATH: &str = "ship_B.png";
const PLAYER_ROTATION_SPEED: f32 = PI;
const PLAYER_SPEED: f32 = 300.0;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
    movement: Movement,
}

impl PlayerBundle {
    fn new(asset_server: Res<AssetServer>) -> Self {
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_input)
            .add_plugins(MovementPlugin);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle::new(asset_server));
}

fn player_input(
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
