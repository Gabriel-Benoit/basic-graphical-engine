use self::collision_algos::{
    get_plane_plane_collision, get_sphere_plane_collision, get_sphere_sphere_collision,
};

use super::collision::Collision;
use crate::engine::common::add_vec3;
use crate::engine::common::transform::Transform;
use ggez::mint::Vector3;

pub trait Collider {
    fn is_colliding(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> bool;
    fn get_collision(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> Option<Collision>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColliderType {
    Sphere(SphereCollider),
    Plane(PlaneCollider),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphereCollider {
    pub position: Vector3<f32>,
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaneCollider {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl Collider for ColliderType {
    fn get_collision(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> Option<Collision> {
        match self {
            Self::Plane(p) => p.get_collision(transfrom, other, other_transfrom),
            Self::Sphere(s) => s.get_collision(transfrom, other, other_transfrom),
        }
    }
    fn is_colliding(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> bool {
        match self {
            Self::Plane(p) => p.is_colliding(transfrom, other, other_transfrom),
            Self::Sphere(s) => s.is_colliding(transfrom, other, other_transfrom),
        }
    }
}

impl Collider for SphereCollider {
    fn is_colliding(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> bool {
        self.get_collision(transfrom, other, other_transfrom)
            .is_some()
    }

    fn get_collision(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> Option<Collision> {
        match other {
            ColliderType::Sphere(sphere) => {
                get_sphere_sphere_collision(self, transfrom, sphere, other_transfrom)
            }
            ColliderType::Plane(p) => {
                get_sphere_plane_collision(self, transfrom, p, other_transfrom)
            }
        }
    }
}

impl Collider for PlaneCollider {
    fn is_colliding(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> bool {
        self.get_collision(transfrom, other, other_transfrom)
            .is_some()
    }

    fn get_collision(
        &self,
        transfrom: Transform,
        other: &ColliderType,
        other_transfrom: Transform,
    ) -> Option<Collision> {
        match other {
            ColliderType::Sphere(sphere) => {
                get_sphere_plane_collision(sphere, other_transfrom, self, transfrom)
            }
            ColliderType::Plane(p) => {
                get_plane_plane_collision(self, transfrom, p, other_transfrom)
            }
        }
    }
}

mod collision_algos {
    use ggez::mint::Vector3;

    use super::{add_vec3, Collision, PlaneCollider, SphereCollider, Transform};

    pub fn get_sphere_plane_collision(
        a: &SphereCollider,
        a_transform: Transform,
        b: &PlaneCollider,
        b_transform: Transform,
    ) -> Option<Collision> {
        let a_pos = add_vec3(a_transform.position, a.position);
        let b_pos = add_vec3(b_transform.position, b.position);
        let distance = (a_pos.x - b_pos.x) * b.normal.x
            + (a_pos.y - b_pos.y) * b.normal.y
            + (a_pos.z - b_pos.z) * b.normal.z;
        if distance <= a.radius {
            let depth = a.radius - distance;
            Some(Collision {
                a: a_pos,
                b: b_pos,
                depth,
                //normal: a.radius - distance,
                is_colliding: true,
            })
        } else {
            None
        }
    }

    pub fn get_sphere_sphere_collision(
        a: &SphereCollider,
        a_transform: Transform,
        b: &SphereCollider,
        b_transform: Transform,
    ) -> Option<Collision> {
        let a_pos = add_vec3(a_transform.position, a.position);
        let b_pos = add_vec3(b_transform.position, b.position);
        let distance =
            (a_pos.x - b_pos.x).powi(2) + (a_pos.y - b_pos.y).powi(2) + (a_pos.z - b_pos.z).powi(2);
        let radius_sum = a.radius + b.radius;
        if distance <= radius_sum.powi(2) {
            let depth = radius_sum - distance.sqrt();
            let _normal = Vector3 {
                x: a_pos.x - b_pos.x,
                y: a_pos.y - b_pos.y,
                z: a_pos.z - b_pos.z,
            };
            Some(Collision {
                a: a_pos,
                b: b_pos,
                depth,
                //normal,
                is_colliding: true,
            })
        } else {
            None
        }
    }

    pub fn get_plane_plane_collision(
        _a: &PlaneCollider,
        _a_transform: Transform,
        _b: &PlaneCollider,
        _b_transform: Transform,
    ) -> Option<Collision> {
        None
    }
}
