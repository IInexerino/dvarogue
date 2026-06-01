use std::collections::HashMap;
use bevy::{ecs::resource::Resource, prelude::{Deref, DerefMut}};
use crate::world::map::{grid::Map, spatial::SpatialMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DungeonKind {
    Dungeon,
    Caves,
}

/// Specifies a dungeon kind and a specific floor
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DungeonFloor {
    pub kind: DungeonKind,
    pub floor: u8
}


impl DungeonFloor { 
    pub fn first_floor(kind: DungeonKind) -> Self {
        DungeonFloor {
            kind,
            floor: 1,
        }
    }
}


/// Resource keeping track of the current floor the player is on
#[derive(Resource, Deref, DerefMut)]
pub struct CurrentFloor(pub DungeonFloor);

/// Resource storing all discovered and generated floors, including current
#[derive(Resource, Deref, DerefMut)]
pub struct DiscoveredFloors(pub HashMap<DungeonFloor, (Map, SpatialMap)>);
