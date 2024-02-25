use bevy::prelude::*;

#[derive(Event)]
pub struct InvincibleEndEvent {
    pub entity: Entity,
}
