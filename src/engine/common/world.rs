use ggez::mint::Vector3;

use super::entity::Entity;

pub struct World {
    pub entities: Vec<Entity>,
}

impl World {
    const GRAVITE: Vector3<f32> = Vector3 {
        x: 0.0,
        y: 9.81,
        z: 0.0,
    };
    pub fn new(_: Self) -> World {
        Self {
            entities: Vec::new(),
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
}
