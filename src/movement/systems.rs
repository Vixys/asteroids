use bevy::prelude::*;
use super::components::*;

pub fn movement_system(mut query: Query<(&mut Movement, &mut Transform)>, time: Res<Time>) {
    for (mut movement, mut transform) in query.iter_mut() {
        if movement.velocity.length() > movement.max_velocity {
            movement.velocity = movement.velocity.clamp_length_max(movement.max_velocity);
        }
        transform.translation += movement.velocity.extend(0.0) * time.delta_seconds();
    }
}