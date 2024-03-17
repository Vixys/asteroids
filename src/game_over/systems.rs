use crate::common::ui::button::{ButtonAction, UiButtonElement};
use crate::constants::MAIN_FONT;
use crate::game_over::components::OnGameOverScreen;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up game over UI");
    let font = asset_server.load(MAIN_FONT);
    commands
        .spawn((
            OnGameOverScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(75.0),
                        height: Val::Percent(50.0),
                        padding: UiRect::horizontal(Val::Px(25.0)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(75.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor::from(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Game Over",
                        TextStyle {
                            font: font.clone(),
                            font_size: 72.0,
                            color: Color::WHITE,
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Percent(5.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.add_button("Restart", font.clone(), ButtonAction::ToInGame);
                            parent.add_button("Back to menu", font.clone(), ButtonAction::ToMenu);
                        });
                });
        });
}
