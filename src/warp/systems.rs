use bevy::prelude::*;

use super::components::*;

pub fn warp_system(
    mut query: Query<&mut Transform, With<Warp>>,
    q_windows: Query<&Window>,
    q_camera: Query<(&GlobalTransform, &Camera)>,
) {
    let window = q_windows.single();
    let (camera_transform, camera) = q_camera.single();

    for mut transform in query.iter_mut() {
        if let Some(mut entity_pos) =
            camera.world_to_viewport(camera_transform, transform.translation)
        {
            if entity_pos.y > window.height() {
                entity_pos.y = 0.0;
            } else if entity_pos.y < 0.0 {
                entity_pos.y = window.height();
            }
            if entity_pos.x > window.width() {
                entity_pos.x = 0.0;
            } else if entity_pos.x < 0.0 {
                entity_pos.x = window.width();
            }

            transform.translation = camera
                .viewport_to_world_2d(camera_transform, entity_pos)
                .unwrap()
                .extend(0.0);
        }
    }
}
