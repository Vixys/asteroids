use std::f32::consts::PI;

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

#[derive(Component, Debug, Clone)]
pub struct AngularVelocity {
    pub max_velocity: f32,
    pub angular_velocity: f32,
}

impl Default for AngularVelocity {
    fn default() -> Self {
        Self {
            max_velocity: PI * 2.0,
            angular_velocity: 0.0,
        }
    }
}
