use bevy::prelude::Component;

#[derive(Debug, Clone, Copy)]
pub enum ColliderShape {
    Circle(f32),
}

#[derive(Component, Debug)]
pub struct Collider {
    pub shape: ColliderShape,
    // Which layers this collider is in
    pub collision_layer: u32,
    // Which layers this collider can collide with
    pub collision_mask: u32,
}
