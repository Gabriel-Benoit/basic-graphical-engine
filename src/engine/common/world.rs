use ggez::mint::Vector3;

use crate::engine::collision::{collider::Collider, collision::Collision, solver::Solver};

use super::entity::Entity;

pub struct World {
    pub entities: Vec<Entity>,
    pub collision: Vec<Collision>,
    pub solvers: Vec<Box<dyn Solver>>,
}

impl World {
    const GRAVITE: Vector3<f32> = Vector3 {
        x: 0.0,
        y: -9.81,
        z: 0.0,
    };
    pub fn new(_: Self) -> World {
        Self {
            entities: Vec::new(),
            collision: Vec::new(),
            solvers: Vec::new(),
        }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.retain(|e| e != &entity);
    }

    pub fn update(&mut self, dt: f32) {
        for entity in &mut self.entities {
            entity.update(dt, Self::GRAVITE);
        }
    }

    pub fn resolve_collisions(&mut self) {
        // Find collisions
        for entity in &self.entities {
            for other in &self.entities {
                if entity == other {
                    continue;
                }
                if entity.collider.is_none() && other.collider.is_none() {
                    continue;
                }
                let collision = entity.collider.unwrap().get_collision(
                    &entity.transform,
                    &other.collider.unwrap(),
                    &other.transform,
                );
                if let Some(c) = collision {
                    self.collision.push(Collision {
                        points: c,
                        a: *entity,
                        b: *other,
                    });
                }
            }
        }
        // Resolve collisions
        for solver in &self.solvers {
            solver.solve(&mut self.collision);
        }
    }

    pub fn add_solver(&mut self, solver: Box<dyn Solver>) {
        self.solvers.push(solver);
    }

    pub fn remove_solver(&mut self, solver: Box<dyn Solver>) {
        self.solvers.retain(|s| s != &solver);
    }

    pub fn start(&mut self) {
        loop {
            self.update(0.1);
            self.resolve_collisions();
        }
    }
}
