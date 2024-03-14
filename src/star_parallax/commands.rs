use bevy::{ecs::system::Command, prelude::*};

use crate::common::{movement::components::Movement, warp::components::Warp};

const STAR_ASSET_PATH: &str = "stars/star_small.png";
const STAR_MAX_SIZE: f32 = 6.0;

#[derive(Debug)]
pub struct SpawnStar {
    pub position: Vec3,
    pub layer: f32,
}

impl Command for SpawnStar {
    fn apply(self, world: &mut World) {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        world.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        STAR_MAX_SIZE / self.layer,
                        STAR_MAX_SIZE / self.layer,
                    )),
                    ..default()
                },
                texture: asset_server.load(STAR_ASSET_PATH),
                transform: Transform::from_translation(self.position),
                ..default()
            },
            Warp,
            Movement {
                velocity: -Vec2::X * 25.0 / self.layer,
                ..default()
            },
        ));
    }
}
