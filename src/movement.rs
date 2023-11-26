use bevy::prelude::*;

// Movement component
#[derive(Component, Debug, Clone)]
pub struct Movement {
    pub max_velocity: f32,
    pub velocity: Vec2,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            max_velocity: 300.0,
            velocity: Vec2::new(0.0, 0.0),
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system);
    }
}

fn movement_system(mut query: Query<(&mut Movement, &mut Transform)>, time: Res<Time>) {
    for (mut movement, mut transform) in query.iter_mut() {
        if movement.velocity.length() > movement.max_velocity {
            movement.velocity = movement.velocity.clamp_length_max(movement.max_velocity);
        }
        transform.translation += movement.velocity.extend(0.0) * time.delta_seconds();
    }
}
