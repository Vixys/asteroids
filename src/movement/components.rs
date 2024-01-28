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