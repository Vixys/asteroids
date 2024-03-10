use bevy::{ecs::system::Command, prelude::*};

#[derive(Debug)]
pub struct SpawnStars {
    number: u32,
    direction: Vec2,
}

impl Command for SpawnStars {
    fn apply(self, world: &mut World) {
        let mut rng = thread_rng();
        world.
        let position = Vec3::new(
            rng.gen_range(-1000.0..1000.0),
            rng.gen_range(-1000.0..1000.0),
            0.0,
        );
        let velocity = self.direction * rng.gen_range(100.0..300.0);
        let color = Color::rgb(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );
        world
            .spawn()
            .insert_bundle(SpriteBundle {
                material: color.into(),
                transform: Transform::from_translation(position),
                sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                ..Default::default()
            })
            .insert(Movement {
                max_velocity: 300.0,
                velocity: velocity,
            });
    }
}
