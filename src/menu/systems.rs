use crate::common::ui::button::UiButtonElements;
use crate::constants::MAIN_FONT;
use crate::menu::components::OnMenuScreen;
use bevy::prelude::*;

pub fn setup_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(MAIN_FONT);
    commands
        .spawn((
            OnMenuScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Asteroids",
                TextStyle {
                    font: font.clone(),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ));
            parent.add_button("Play", font.clone());
        });
}
