use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::common::movement::components::Movement;
use crate::constants::{PLAYER_COLLISION_LAYER, ZERO_COLLISION_LAYER};
use crate::game_state::GameState;
use crate::in_game::blink::components::Blink;
use crate::in_game::bullet::commands::SpawnBullet;
use crate::in_game::collider::components::*;
use crate::in_game::collider::events::CollisionEvent;
use crate::in_game::invincible::components::Invincible;
use crate::in_game::invincible::events::InvincibleEndEvent;
use crate::in_game::player::commands::SpawnPlayer;
use crate::in_game::player::constants::*;

use super::components::*;

pub fn spawn_player(mut commands: Commands) {
    commands.add(SpawnPlayer::default())
}

pub fn player_input(mut commands: Commands, mut query: Query<(&mut Movement, &mut Transform, &mut Player)>, keyboard_input: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    for (mut movement, mut player_transform, mut player) in query.iter_mut() {
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
            let direction = Vec2::from_angle(player_transform.rotation.to_scaled_axis().z + FRAC_PI_2).normalize();
            movement.velocity += direction * PLAYER_SPEED * time.delta_seconds();
        }
        if player.fire_cooldown.tick(time.delta()).finished() && keyboard_input.just_pressed(KeyCode::Space) {
            info!("Bullet spawned");
            commands.add(SpawnBullet { transform: *player_transform });
            player.fire_cooldown.reset();
        }
    }
}

pub fn on_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Movement, &mut PlayerLives), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in collision_events.read() {
        let entity2 = query.get_mut(event.entity2);

        if let Ok((entity, mut transform, mut movement, mut lives)) = entity2 {
            // TODO: It can happend that lives is 0, so we should check for that
            // Otherwise it leads to panic: attempt to subtract with overflow
            lives.0 -= 1;
            if lives.0 == 0 {
                commands.entity(entity).despawn();
                next_state.set(GameState::GameOver);
            } else {
                transform.translation = Vec3::ZERO;
                transform.rotation = Quat::from_rotation_z(0.0);
                movement.velocity = Vec2::ZERO;
                commands.entity(entity).insert(Invincible::default());
                commands.entity(entity).insert(Blink::default());
                commands.entity(entity).remove::<Collider>();
            }
        }
    }
    collision_events.clear();
}

pub fn on_invincibility_end_system(mut commands: Commands, mut invincibility_end_events: EventReader<InvincibleEndEvent>, query: Query<Entity, (With<Player>, Without<Collider>)>) {
    for event in invincibility_end_events.read() {
        if let Ok(entity) = query.get(event.entity) {
            info!("Add collider to {:?}!", entity);
            commands.entity(entity).remove::<Blink>();
            commands.entity(entity).insert(Collider {
                shape: ColliderShape::Circle(16.0),
                collision_layer: PLAYER_COLLISION_LAYER,
                collision_mask: ZERO_COLLISION_LAYER,
            });
        }
    }
    invincibility_end_events.clear();
}
