mod components;
mod systems;

use crate::game_state::GameState;
use crate::menu::components::OnMenuScreen;
use crate::menu::systems::*;
use crate::systems::despawn_all;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu_ui)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_all::<OnMenuScreen>);
    }
}
