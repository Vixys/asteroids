use bevy::prelude::*;

#[derive(Event)]
pub struct CollisionEvent {
    pub entity1: Entity,
    pub entity2: Entity,
}
