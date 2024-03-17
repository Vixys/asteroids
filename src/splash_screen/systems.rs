use crate::game_state::GameState;
use crate::splash_screen::components::*;
use bevy::prelude::*;

const SPLASH_SCREEN_LOGO: &str = "imp_game_logo.png";

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            OnSplashScreen,
            SwitchToMenuScreen {
                timer: Timer::from_seconds(5.0, TimerMode::Once),
            },
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load(SPLASH_SCREEN_LOGO),
                    ..Default::default()
                },
                style: Style { ..default() },
                ..default()
            });
        });
}

pub fn go_to_menu_screen(mut next_state: ResMut<NextState<GameState>>, mut query: Query<&mut SwitchToMenuScreen>, time: Res<Time>) {
    let mut timer = query.single_mut();

    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        next_state.set(GameState::Menu);
    }
}
