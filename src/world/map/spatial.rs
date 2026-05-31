use std::collections::HashMap;

use bevy::{ecs::entity::Entity, math::IVec2};

use crate::world::map::grid::MapSize;


pub struct SpatialMap {
    pub entities: HashMap<IVec2, Vec<Entity>>,
}

impl SpatialMap {
    pub fn new_empty(map_size: &MapSize) -> Self {
        let mut hashmap = HashMap::new();

        for y in 0..map_size.height {
            for x in 0..map_size.width {
                hashmap.insert(IVec2::new(x, y), Vec::new());
            }
        }

        SpatialMap { entities: hashmap }
    }
}
