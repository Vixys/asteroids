use bevy::prelude::*;

mod asteroid;
mod collider;
mod movement;
mod player;
mod warp;

use asteroid::AsteroidPlugin;
use collider::ColliderPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(ColliderPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
