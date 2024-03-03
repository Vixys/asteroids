use crate::game_over::components::OnGameOverScreen;
use crate::game_over::systems::setup_ui;
use crate::game_state::GameState;
use crate::systems::despawn_all;
use bevy::prelude::*;

mod components;
mod systems;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_ui)
            // .add_systems(Update, button_system.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), despawn_all::<OnGameOverScreen>);
    }
}
