use ggez::mint::Vector3;

pub struct Collision {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub depth: f32,
    pub is_colliding: bool,
}
