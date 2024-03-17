use bevy::prelude::*;

use crate::in_game::collider::events::CollisionEvent;

use super::components::Bullet;

pub fn despawn_out_of_bounds(mut commands: Commands, q_windows: Query<&Window>, q_camera: Query<(&GlobalTransform, &Camera)>, query: Query<(Entity, &Transform), With<Bullet>>) {
    let window = q_windows.single();
    let (camera_transform, camera) = q_camera.single();
    let window_rect = Rect::from_corners(Vec2::ZERO, Vec2::from_array([window.width(), window.height()]));

    for (entity, transform) in query.iter() {
        if let Some(entity_pos) = camera.world_to_viewport(camera_transform, transform.translation) {
            if !window_rect.contains(entity_pos) {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn on_collision(mut commands: Commands, mut events: EventReader<CollisionEvent>, query: Query<Entity, With<Bullet>>) {
    for event in events.read() {
        let entity1 = query.get(event.entity1);
        let entity2 = query.get(event.entity2);

        if let Ok(entity) = entity1 {
            commands.entity(entity).despawn();
        }

        if let Ok(entity) = entity2 {
            commands.entity(entity).despawn();
        }
    }
    events.clear();
}
