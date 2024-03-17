use bevy::prelude::*;

pub trait Rotate2D {
    fn rotate_2d(&self, angle: f32) -> Self;
}

impl Rotate2D for Vec2 {
    /// Rotate a 2D vector by an angle in radians
    fn rotate_2d(&self, angle: f32) -> Self {
        Quat::from_rotation_z(angle).mul_vec3(self.extend(0.0)).truncate().normalize()
    }
}
