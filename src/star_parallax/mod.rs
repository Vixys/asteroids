use crate::game_state::GameState;
use crate::star_parallax::systems::setup;
use bevy::prelude::*;

mod commands;
mod components;
mod systems;

pub struct StarParallaxPlugin;

impl Plugin for StarParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup);
    }
}
