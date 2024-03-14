use bevy::{prelude::*, window::PrimaryWindow};
use rand::{rngs::ThreadRng, thread_rng, Rng};

use super::commands::SpawnStar;

const STAR_NUMBER_L1: u32 = 256;
const STAR_NUMBER_L2: u32 = 512;
const STAR_NUMBER_L3: u32 = 1024;

pub fn setup(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let mut rng = thread_rng();

    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();

    let mut stars = spawn_star_layer(
        STAR_NUMBER_L3,
        window,
        3,
        &mut rng,
        camera,
        camera_transform,
    );

    stars.append(&mut spawn_star_layer(
        STAR_NUMBER_L2,
        window,
        2,
        &mut rng,
        camera,
        camera_transform,
    ));

    stars.append(&mut spawn_star_layer(
        STAR_NUMBER_L1,
        window,
        1,
        &mut rng,
        camera,
        camera_transform,
    ));

    for star in stars {
        commands.add(star);
    }
}

fn spawn_star_layer(
    start_number: u32,
    window: &Window,
    layer: u32,
    rng: &mut ThreadRng,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec<SpawnStar> {
    let mut stars = Vec::new();

    let spawn_area = Vec2::splat((window.width() * window.height() / start_number as f32).sqrt());
    for x in 0..(window.width() / spawn_area.x) as u32 {
        for y in 0..(window.height() / spawn_area.y) as u32 {
            let position = Vec2::new(
                x as f32 * spawn_area.x + rng.gen_range(0.0..spawn_area.x),
                y as f32 * spawn_area.y + rng.gen_range(0.0..spawn_area.y),
            );
            let pos_3d = camera
                .viewport_to_world_2d(camera_transform, position)
                .unwrap()
                .extend(0.0);

            stars.push(SpawnStar {
                position: pos_3d,
                layer: layer as f32,
            });
        }
    }

    stars
}
