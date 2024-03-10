use crate::common::movement::components::Movement;
use crate::common::warp::components::Warp;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{thread_rng, Rng};

const STAR_NUMBER: u32 = 1000;
const STAR_ASSET_PATH: &str = "stars/star_small.png";

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let mut rng = thread_rng();

    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();

    for _ in 0..STAR_NUMBER {
        let position = Vec2::new(
            rng.gen_range(0.0..window.width()),
            rng.gen_range(0.0..window.height()),
        );
        let pos_3d = camera
            .viewport_to_world_2d(camera_transform, position)
            .unwrap()
            .extend(0.0);
        let layer = rng.gen_range(1..=3);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(8.0 / layer as f32, 8.0 / layer as f32)),
                    ..default()
                },
                texture: asset_server.load(STAR_ASSET_PATH),
                transform: Transform::from_translation(pos_3d),
                ..default()
            },
            Warp,
            Movement {
                velocity: -Vec2::X * 25.0 / layer as f32,
                ..default()
            },
        ));
    }
}
