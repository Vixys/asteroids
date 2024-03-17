use crate::game_state::GameState;
use crate::splash_screen::components::*;
use crate::splash_screen::systems::{go_to_menu_screen, setup_ui};
use crate::systems::despawn_all;
use bevy::prelude::*;

mod components;
mod systems;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), setup_ui)
            .add_systems(Update, go_to_menu_screen.run_if(in_state(GameState::SplashScreen)))
            .add_systems(OnExit(GameState::SplashScreen), despawn_all::<OnSplashScreen>);
    }
}
