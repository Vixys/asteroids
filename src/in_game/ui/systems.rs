use crate::in_game::components::OnInGameScreen;
use crate::in_game::player::components::*;
use crate::in_game::player::constants::PLAYER_SHIP_ASSET_PATH;
use crate::in_game::ui::components::UiLive;
use bevy::prelude::*;

pub fn setup_in_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            OnInGameScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(24.0),
                    padding: UiRect {
                        // Rotating is not taken into account currently in bevy Flex
                        // Have to set the padding manually
                        left: Val::Px(16.0),
                        ..default()
                    },
                    ..Default::default()
                },
                background_color: BackgroundColor::from(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            ui_live(parent, &asset_server);
            ui_live(parent, &asset_server);
            ui_live(parent, &asset_server);
        });
}

pub fn update_ui(
    mut commands: Commands,
    q_ui_lives: Query<Entity, With<UiLive>>,
    q_player_live: Query<&PlayerLives, Changed<PlayerLives>>,
) {
    for live in q_player_live.iter() {
        for entity in q_ui_lives.iter().skip(live.0 as usize) {
            commands.entity(entity).despawn();
        }
    }
}

fn ui_live(builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    builder.spawn((
        UiLive,
        ImageBundle {
            transform: Transform {
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            image: UiImage {
                texture: asset_server.load(PLAYER_SHIP_ASSET_PATH),
                ..Default::default()
            },
            style: Style { ..default() },
            ..default()
        },
    ));
}
