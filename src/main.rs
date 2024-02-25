use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod constants;
mod game_state;
mod helper;
mod in_game;
mod systems;

use game_state::GameState;
use in_game::InGamePlugin;
use systems::setup;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(InGamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .run();
}
