use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod common;
mod constants;
mod game_over;
mod game_state;
mod helper;
mod in_game;
mod menu;
mod systems;

use common::CommonPlugin;
use game_over::GameOverPlugin;
use game_state::GameState;
use in_game::InGamePlugin;
use menu::MenuPlugin;
use systems::setup;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(InGamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}
