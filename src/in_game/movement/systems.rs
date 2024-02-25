use super::components::*;
use bevy::prelude::*;

pub fn movement_system(mut query: Query<(&mut Movement, &mut Transform)>, time: Res<Time>) {
    for (mut movement, mut transform) in query.iter_mut() {
        if movement.velocity.length() > movement.max_velocity {
            movement.velocity = movement.velocity.clamp_length_max(movement.max_velocity);
        }
        transform.translation += movement.velocity.extend(0.0) * time.delta_seconds();
    }
}

pub fn angular_velocity_system(
    mut query: Query<(&mut AngularVelocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            velocity.angular_velocity * time.delta_seconds(),
        ));
    }
}
