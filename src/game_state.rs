use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    SplashScreen,
    Menu,
    #[default]
    InGame,
    GameOver,
}
