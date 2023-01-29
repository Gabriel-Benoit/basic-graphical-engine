use ggez::mint::Quaternion;
use ggez::mint::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}
