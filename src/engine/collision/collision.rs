use ggez::mint::Vector3;

use crate::engine::common::entity::Entity;

pub struct CollisionPoints {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub depth: f32,
    pub is_colliding: bool,
}

pub struct Collision {
    pub points: CollisionPoints,
    pub a: Entity,
    pub b: Entity,
}
