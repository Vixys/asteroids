use bevy::app::Plugin;
use bevy::prelude::*;

pub mod asteroid;
mod blink;
mod bullet;
pub mod collider;
mod invincible;
mod movement;
pub mod player;
mod warp;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(asteroid::AsteroidPlugin)
            .add_plugins(collider::ColliderPlugin)
            .add_plugins(player::PlayerPlugin);
    }
}
