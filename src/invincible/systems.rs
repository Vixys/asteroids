use bevy::prelude::*;

use super::{components::Invincible, events::InvincibleEndEvent};

pub fn fade_invincible(
    mut commands: Commands,
    time: Res<Time>,
    mut event_writer: EventWriter<InvincibleEndEvent>,
    mut query: Query<(Entity, &mut Invincible)>,
) {
    for (entity, mut invincible) in query.iter_mut() {
        if invincible.fade_timer.tick(time.delta()).just_finished() {
            info!("{:?}: invincibility ends", entity);
            commands.entity(entity).remove::<Invincible>();
            event_writer.send(InvincibleEndEvent { entity });
        }
    }
}

pub fn blink_invincible(mut query: Query<(&mut Invincible, &mut Visibility)>, time: Res<Time>) {
    for (mut invicibility, mut visible) in query.iter_mut() {
        if invicibility.blink_timer.tick(time.delta()).just_finished() {
            *visible = match *visible {
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Hidden,
            };
        }
    }
}

pub fn invincible_end_system(
    mut event_reader: EventReader<InvincibleEndEvent>,
    mut query: Query<&mut Visibility>,
) {
    for event in event_reader.read() {
        if let Ok(mut visibility) = query.get_mut(event.entity) {
            *visibility = Visibility::Visible;
        }
    }
}
