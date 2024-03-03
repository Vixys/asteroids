use bevy::prelude::*;

use crate::game_state::GameState;
use crate::in_game::ui::systems::*;

mod commands;
mod components;
mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_in_game_ui)
            .add_systems(Update, update_ui.run_if(in_state(GameState::InGame)));
    }
}
