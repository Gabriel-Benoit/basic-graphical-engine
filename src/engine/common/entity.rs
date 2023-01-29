use crate::engine::collision::collider::ColliderType;
use ggez::mint::Quaternion;
use ggez::mint::Vector3;

use super::{add_vec3, mul_vec3, transform::Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entity {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub force: Vector3<f32>,
    pub mass: f32,
    pub transform: Transform,
    pub collider: Option<ColliderType>,
}

impl Entity {
    pub fn new(
        position: Vector3<f32>,
        velocity: Vector3<f32>,
        force: Vector3<f32>,
        mass: f32,
    ) -> Entity {
        Self {
            position,
            velocity,
            force,
            mass,
            transform: Transform {
                position,
                scale: Vector3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                rotation: Quaternion {
                    s: 1.0,
                    v: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
            },
            collider: None,
        }
    }
    pub fn update(&mut self, dt: f32, gravity: Vector3<f32>) {
        // a = gravity
        // v = v0 + at
        self.velocity = add_vec3(self.velocity, mul_vec3(gravity, dt));
        // p = p0 + vt
        self.position = add_vec3(self.position, mul_vec3(self.velocity, dt));
    }
}
