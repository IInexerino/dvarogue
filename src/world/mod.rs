pub mod floor;
pub mod map;
pub mod systems;

use bevy::{ecs::entity::Entity, math::IVec2};
use thiserror::Error;
use crate::world::floor::DungeonFloor;

/// Errors related to this and any child modules
#[derive(Error, Debug)]
pub enum MapError {
    #[error("Floor '{0:?}' invalid" )]
    InvalidFloorSpecification(DungeonFloor),
    #[error("Tile idx {0} is out of bounds" )]
    OutOfBoundsTileIdx(usize),
    #[error("Position {0:?} does not exist in SpatialMap")]
    SpatialMapMissingPosition(IVec2),
    #[error("Entity {0:?} not present in positional tile {1:?} in SpatialMap")]
    SpatialMapAbscentEntity(Entity, IVec2)
}