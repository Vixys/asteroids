use crate::in_game::bullet::bundles::BulletBundle;
use crate::in_game::bullet::constants::{BULLET_ASSET_PATH, BULLET_SPEED};
use crate::in_game::components::OnInGameScreen;
use bevy::ecs::system::Command;
use bevy::prelude::*;

#[derive(Debug)]
pub struct SpawnBullet {
    pub transform: Transform,
}

impl Command for SpawnBullet {
    fn apply(self, world: &mut bevy::ecs::world::World) {
        let asset_server = world.get_resource::<bevy::asset::AssetServer>().unwrap();
        let mut bullet = BulletBundle::default();

        bullet.sprite.transform = self.transform;
        bullet.sprite.texture = asset_server.load(BULLET_ASSET_PATH);
        bullet.movement.velocity = self.transform.rotation.mul_vec3(Vec3::Y).truncate().normalize() * BULLET_SPEED;

        world.spawn((bullet, OnInGameScreen));
    }
}
