use bevy::prelude::*;

use super::{components::Invincible, events::InvincibleEndEvent};

pub fn fade_invincible(mut commands: Commands, time: Res<Time>, mut event_writer: EventWriter<InvincibleEndEvent>, mut query: Query<(Entity, &mut Invincible)>) {
    for (entity, mut invincible) in query.iter_mut() {
        if invincible.fade_timer.tick(time.delta()).just_finished() {
            info!("{:?}: invincibility ends", entity);
            commands.entity(entity).remove::<Invincible>();
            event_writer.send(InvincibleEndEvent { entity });
        }
    }
}
