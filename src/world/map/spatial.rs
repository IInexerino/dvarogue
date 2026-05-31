use std::collections::{HashMap, hash_map::Entry};
use bevy::{ecs::entity::Entity, math::IVec2};

use crate::world::MapError;

pub struct SpatialMap {
    pub entities: HashMap<IVec2, Vec<Entity>>,
}

impl SpatialMap {
    pub fn new() -> Self {
        SpatialMap { entities: HashMap::new() }
    }

    pub fn push_to_position(&mut self, pos: IVec2, entity: Entity) {
        match self.entities.entry(pos) {
            Entry::Occupied(mut e) => { e.get_mut().push(entity); },
            Entry::Vacant(e) => { e.insert(vec![entity]); },
        };
    }

    
    pub fn remove_from_position(&mut self, pos: &IVec2, entity: Entity) -> Result<(), MapError> {
        let pos_vec = self.entities
            .get_mut(pos)
            .ok_or(MapError::SpatialMapMissingPosition(*pos))?;

        let entity_idx = pos_vec
            .iter().position(| &ent| ent == entity )
            .ok_or(MapError::SpatialMapAbscentEntity(entity, *pos))?;

        pos_vec.remove(entity_idx);

        if pos_vec.is_empty() {
            self.entities.remove(pos).unwrap();
        }

        Ok(())
    }

    pub fn move_entity(&mut self, origin: &IVec2, destination: IVec2, entity: Entity) -> Result<(), MapError> {
        self.remove_from_position(origin, entity)?;
        self.push_to_position(destination, entity);
        Ok(())
    }
}
