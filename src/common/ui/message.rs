use bevy::prelude::*;

pub trait UiMessageElement {
    fn add_message_box(&mut self, text: &str, font: Handle<Font>);
}

impl UiMessageElement for ChildBuilder<'_> {
    fn add_message_box(&mut self, text: &str, font: Handle<Font>) {
        self.spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(25.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor::from(Color::rgba(0.0, 0.0, 0.0, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 72.0,
                    color: Color::WHITE,
                },
            ));
        });
    }
}
