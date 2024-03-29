use bevy::prelude::*;
#[cfg(not(target_family = "wasm"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod common;
mod constants;
mod game_over;
mod game_state;
mod helper;
mod in_game;
mod menu;
mod splash_screen;
mod star_parallax;
mod systems;

use crate::splash_screen::SplashScreenPlugin;
use common::CommonPlugin;
use game_over::GameOverPlugin;
use game_state::GameState;
use in_game::InGamePlugin;
use menu::MenuPlugin;
use star_parallax::StarParallaxPlugin;
use systems::setup;

fn main() {
    let mut app = App::new();
    app.init_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb_u8(1, 11, 25)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(SplashScreenPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(StarParallaxPlugin)
        .add_systems(Startup, setup)
        .add_plugins(InGamePlugin);
    #[cfg(not(target_family = "wasm"))]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}
