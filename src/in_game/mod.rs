use crate::game_state::GameState;
use crate::in_game::components::OnInGameScreen;
use crate::systems::despawn_all;
use bevy::app::Plugin;
use bevy::prelude::*;

pub mod asteroid;
mod blink;
mod bullet;
pub mod collider;
mod components;
mod invincible;
mod level;
pub mod player;
mod ui;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(asteroid::AsteroidPlugin)
            .add_plugins(collider::ColliderPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(ui::UiPlugin)
            .add_plugins(level::LevelPlugin)
            .add_systems(OnExit(GameState::InGame), despawn_all::<OnInGameScreen>);
    }
}
