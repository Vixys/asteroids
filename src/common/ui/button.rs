use crate::game_state::GameState;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5);

pub trait UiButtonElements {
    fn add_button(&mut self, text: &str, font: Handle<Font>, action: ButtonAction);
}

#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum ButtonAction {
    ToInGame,
    ToMenu,
}

impl UiButtonElements for ChildBuilder<'_> {
    fn add_button(&mut self, text: &str, font: Handle<Font>, action: ButtonAction) {
        self.spawn((
            ButtonBundle {
                style: Style {
                    min_height: Val::Px(75.0),
                    min_width: Val::Px(250.0),
                    padding: UiRect::horizontal(Val::Px(5.0)),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            action,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        });
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                match action {
                    ButtonAction::ToInGame => next_state.set(GameState::InGame),
                    ButtonAction::ToMenu => next_state.set(GameState::Menu),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
