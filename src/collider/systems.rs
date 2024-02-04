use bevy::prelude::*;

use super::components::*;
use super::events::*;

pub fn check_collision(
    mut collision_events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Transform, &Collider)>,
) {
    let mut entities = Vec::new();
    let mut transforms = Vec::new();
    let mut colliders = Vec::new();
    for (entity, transform, collider) in query.iter() {
        entities.push(entity);
        transforms.push(transform);
        colliders.push(collider);
    }
    for i in 0..entities.len() {
        for j in 0..entities.len() {
            if i == j || colliders[j].collision_layer & colliders[i].collision_mask == 0 {
                continue;
            }

            match (colliders[i].shape, colliders[j].shape) {
                (ColliderShape::Circle(radius1), ColliderShape::Circle(radius2)) => {
                    let distance = Vec2::distance(
                        transforms[i].translation.truncate(),
                        transforms[j].translation.truncate(),
                    );
                    if distance < radius1 + radius2 {
                        collision_events.send(CollisionEvent {
                            entity1: entities[i],
                            entity2: entities[j],
                        });
                    }
                }
            }
        }
    }
}
