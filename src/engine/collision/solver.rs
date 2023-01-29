use ggez::mint::Vector3;

use super::collision::Collision;
use crate::engine::common::{entity::Entity, *};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BasicSolver;

//#[derive(Debug, Clone, Copy, PartialEq)]
pub trait Solver {
    fn solve(&self, collisions: &mut Vec<Collision>) -> ();
}

impl Solver for BasicSolver {
    fn solve(&self, collisions: &mut Vec<Collision>) -> () {
        for collision in collisions {
            let a = &mut collision.a;
            let b = &mut collision.b;
            let points = &collision.points;
            if points.is_colliding {
                a.velocity = get_v_hat(a, b);
                b.velocity = get_v_hat(b, a);
            }
        }
    }
}

impl PartialEq for dyn Solver + '_ {
    #[allow(unconditional_recursion)]
    fn eq(&self, that: &dyn Solver) -> bool {
        <dyn Solver>::eq(self, that)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq<dyn Solver> for Box<dyn Solver + '_> {
    fn eq(&self, that: &dyn Solver) -> bool {
        <dyn Solver>::eq(&**self, that)
    }
}

fn get_v_hat(a: &mut Entity, b: &mut Entity) -> Vector3<f32> {
    let a_mass = a.mass;
    let b_mass = b.mass;
    let a_velocity = a.velocity;
    let b_velocity = b.velocity;
    let a_position = a.position;
    let b_position = b.position;

    sub_vec3(
        a_velocity,
        mul_vec3(
            sub_vec3(a_position, b_position),
            (2.0 * a_mass) / (a_mass + b_mass)
                * dot_vec3(
                    sub_vec3(a_velocity, b_velocity),
                    sub_vec3(a_position, b_position),
                )
                / norm_of_vec3(sub_vec3(a_position, b_position)),
        ),
    )
}
