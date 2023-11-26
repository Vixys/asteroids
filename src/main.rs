use std::f32::consts::{FRAC_1_PI, FRAC_2_PI, PI};

use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
        }
    }
}

impl PlayerBundle {
    fn new() -> Self {
        PlayerBundle::default()
    }
}

struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, player_look_at_mouse));
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::A) {
        direction.x = -1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction.x = 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction.y = 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction.y = -1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }
    player_transform.translation += direction.extend(0.0) * 300.0 * time.delta_seconds();
}

fn player_look_at_mouse(
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();

    if let Some(mouse_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let angle = (player_transform.translation.xy() - mouse_position).angle_between(Vec2::X);

        player_transform.rotation = Quat::from_rotation_z(-angle);
    }
}

fn player_rotate(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let mut player_transform = query.single_mut();

    player_transform.rotate_z(PI * time.delta_seconds());
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle::new());
}

fn main() {
    App::new().add_plugins((DefaultPlugins, PlayerPlugin)).run();
}
