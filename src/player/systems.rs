use bevy::prelude::*;

use std::f32::consts::FRAC_PI_2;
use super::components::*;
use crate::bullet::bundles::BulletBundle;
use crate::collider::components::*;
use crate::collider::events::CollisionEvent;
use crate::constants::{PLAYER_COLLISION_LAYER, ZERO_COLLISION_LAYER};
use crate::invincible::events::InvincibleEndEvent;
use crate::movement::components::Movement;
use crate::player::commands::SpawnPlayer;
use crate::player::constants::*;

pub fn spawn_player(mut commands: Commands) {
    commands.add(SpawnPlayer::default())
}

pub fn player_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Movement, &mut Transform), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut movement, mut player_transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player_transform.rotate_z(PLAYER_ROTATION_SPEED * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player_transform.rotate_z(-PLAYER_ROTATION_SPEED * time.delta_seconds());
        }

        if keyboard_input.pressed(KeyCode::KeyR) {
            player_transform.rotation = Quat::from_rotation_z(0.0);
            player_transform.translation = Vec3::ZERO;
            movement.velocity = Vec2::ZERO;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            let direction =
                Vec2::from_angle(player_transform.rotation.to_scaled_axis().z + FRAC_PI_2)
                    .normalize();
            movement.velocity += direction * PLAYER_SPEED * time.delta_seconds();
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            let bullet = BulletBundle::new(&player_transform, &asset_server);
            info!("Bullet spawned");
            commands.spawn(bullet);
        }
    }
}

pub fn on_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, With<Player>>,
) {
    for event in collision_events.read() {
        let entity2 = query.get(event.entity2);

        if let Ok(entity) = entity2 {
            commands.entity(entity).despawn();
            commands.add(SpawnPlayer::default());
        }
    }
}

pub fn on_invincibility_end_system(
    mut commands: Commands,
    mut invincibility_end_events: EventReader<InvincibleEndEvent>,
    query: Query<(Entity, &Visibility), (With<Player>, Without<Collider>)>,
) {
    for event in invincibility_end_events.read() {
        if let Ok((entity, visibilty)) = query.get(event.entity) {
            info!("Add collider to {:?}!", entity);
            commands.entity(entity).insert(Collider {
                shape: ColliderShape::Circle(16.0),
                collision_layer: PLAYER_COLLISION_LAYER,
                collision_mask: ZERO_COLLISION_LAYER,
            });
        }
    }
}
