use bevy::prelude::*;

mod asteroid;
mod movement;
mod player;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(asteroid::AsteroidPlugin)
        .run();
}
