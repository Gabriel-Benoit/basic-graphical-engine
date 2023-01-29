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

pub fn sub_vec3(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
    Vector3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

pub fn normal_of_vec3(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
    let initial_vec = sub_vec3(a, b);
    // find normal such that multiplying initial_vec by normal gives a null vector
    // (i.e. the normal is perpendicular to the initial vector)
    Vector3 {
        x: -initial_vec.z,
        y: initial_vec.y,
        z: initial_vec.x,
    }
}

pub fn dot_vec3(a: Vector3<f32>, b: Vector3<f32>) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn norm_of_vec3(a: Vector3<f32>) -> f32 {
    (a.x * a.x + a.y * a.y + a.z * a.z).sqrt()
}
