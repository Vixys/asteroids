use bevy::prelude::*;

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Component)]
pub struct SwitchToMenuScreen {
    pub timer: Timer,
}
