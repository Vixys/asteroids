use bevy::prelude::*;

use crate::common::ui::message::UiMessageElement;
use crate::constants::MAIN_FONT;
use crate::in_game::asteroid::commands::SpawnAsteroid;
use crate::in_game::asteroid::components::Asteroid;
use crate::in_game::asteroid::components::AsteroidSize;
use crate::in_game::components::OnInGameScreen;

use super::components::*;
use super::level_state::LevelState;
use super::resources::LevelStateTimer;

pub fn setup_level(mut commands: Commands, mut next_state: ResMut<NextState<LevelState>>) {
    info!("Spawning Level...");
    commands.spawn((OnInGameScreen, AsteroidLevel::default()));
    next_state.set(LevelState::Start);
}

pub fn start_level_ui(commands: Commands, mut q_level: Query<&mut AsteroidLevel>, mut state_timer: ResMut<LevelStateTimer>, asset_server: Res<AssetServer>) {
    info!("Showing start level UI...");
    let mut level = q_level.single_mut();
    level.spawning_timer.reset();

    create_message(format!("LEVEL {}", level.id).as_str(), commands, asset_server);

    state_timer.next_state(LevelState::InProgress);
}

pub fn spawn_initial_asteroids(mut commands: Commands, q_level: Query<&AsteroidLevel>) {
    let level = q_level.single();
    info!("Spawning initial asteroids...");
    for _ in 0..level.initial_asteroids {
        commands.add(SpawnAsteroid::random().with_size(AsteroidSize::Big));
    }
}

pub fn spawn_asteroid(mut commands: Commands, time: Res<Time>, q_asteroids: Query<Entity, With<Asteroid>>, mut q_level: Query<&mut AsteroidLevel>) {
    if let Ok(mut level) = q_level.get_single_mut() {
        if level.total_asteroids > 0 && q_asteroids.iter().count() < level.max_asteroids as usize && level.spawning_timer.tick(time.delta()).finished() {
            commands.add(SpawnAsteroid::random().with_max_speed(level.asteroid_max_speed));
            level.total_asteroids -= 1;
            level.spawning_timer.reset();
        }
    }
}

pub fn level_completed(
    commands: Commands,
    q_asteroids: Query<Entity, With<Asteroid>>,
    mut q_level: Query<&mut AsteroidLevel>,
    mut state_timer: ResMut<LevelStateTimer>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    if let Ok(mut level) = q_level.get_single_mut() {
        if q_asteroids.iter().count() == 0 && level.total_asteroids == 0 {
            info!("Level completed!");
            next_state.set(LevelState::End);

            create_message(format!("LEVEL {} FINISHED", level.id).as_str(), commands, asset_server);

            level.next_level();
            state_timer.next_state(LevelState::Start);
        }
    }
}

pub fn update_level_state(
    mut commands: Commands,
    q: Query<Entity, With<LevelUiMarker>>,
    time: Res<Time>,
    mut state_timer: ResMut<LevelStateTimer>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    if let Some(timer) = state_timer.timer.as_mut() {
        if timer.tick(time.delta()).finished() {
            next_state.set(state_timer.next_state.unwrap());
            state_timer.reset();
            info!("Clearing level UI...");
            for e in q.iter() {
                commands.entity(e).despawn_recursive();
            }
        }
    }
}

// Helpers

fn create_message(text: &str, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            LevelUiMarker,
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
            parent.add_message_box(text, asset_server.load(MAIN_FONT));
        });
}
