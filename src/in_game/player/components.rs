use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub fire_cooldown: Timer,
}

#[derive(Component)]
pub struct PlayerLives(pub u32);
