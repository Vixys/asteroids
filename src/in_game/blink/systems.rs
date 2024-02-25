use super::components::Blink;
use bevy::log::info;
use bevy::prelude::{Query, RemovedComponents, Res, Time, Visibility};

pub fn blink_system(mut query: Query<(&mut Blink, &mut Visibility)>, time: Res<Time>) {
    for (mut invicibility, mut visible) in query.iter_mut() {
        if invicibility.timer.tick(time.delta()).just_finished() {
            *visible = match *visible {
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Hidden,
            };
        }
    }
}

pub fn blink_removed(mut removed: RemovedComponents<Blink>, mut query: Query<&mut Visibility>) {
    for entity in removed.read() {
        if let Ok(mut visibility) = query.get_mut(entity) {
            info!("Blink removed");
            *visibility = Visibility::Inherited;
        }
    }
}
