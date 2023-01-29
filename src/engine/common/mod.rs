use ggez::mint::Vector3;

pub mod entity;
pub mod transform;
pub mod world;

pub fn mul_vec3(a: Vector3<f32>, b: f32) -> Vector3<f32> {
    Vector3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
}

pub fn div_vec3(a: Vector3<f32>, b: f32) -> Vector3<f32> {
    Vector3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
}

pub fn add_vec3(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
    Vector3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}
