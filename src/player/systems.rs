use bevy::prelude::*;

use std::f32::consts::{FRAC_PI_2, PI};

use crate::bullet::bundles::BulletBundle;
use crate::collider::events::CollisionEvent;
use crate::movement::components::Movement;

use super::bundles::PlayerBundle;
use super::components::*;
use super::events::SpawnPlayerEvent;

const PLAYER_ROTATION_SPEED: f32 = PI;
const PLAYER_SPEED: f32 = 300.0;

pub fn spawn_player(mut spawn_events: EventWriter<SpawnPlayerEvent>) {
    spawn_events.send(SpawnPlayerEvent);
}

pub fn player_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

        if keyboard_input.just_pressed(KeyCode::Space) {
            let bullet = BulletBundle::new(&player_transform, &asset_server);
            println!("Bullet spawned");
            commands.spawn(bullet);
        }
    }
}

pub fn on_collistion_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut spawn_events: EventWriter<SpawnPlayerEvent>,
    query: Query<Entity, With<Player>>,
) {
    for event in collision_events.read() {
        let entity2 = query.get(event.entity2);

        if let Ok(entity) = entity2 {
            commands.entity(entity).despawn();
            spawn_events.send(SpawnPlayerEvent);
        }
    }
}

pub fn on_spawn_player_system(
    mut commands: Commands,
    spawn_events: EventReader<SpawnPlayerEvent>,
    asset_server: Res<AssetServer>,
) {
    if !spawn_events.is_empty() {
        commands.spawn(PlayerBundle::new(asset_server));
    }
}
