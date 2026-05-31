pub mod floor;
pub mod map;
pub mod systems;

use thiserror::Error;
use crate::world::floor::DungeonFloor;

/// Errors related to this and any child modules
#[derive(Error, Debug)]
pub enum MapError {
    #[error("Floor '{0:?}' invalid" )]
    InvalidFloorSpecification(DungeonFloor),
    #[error("Tile idx {0} is out of bounds" )]
    OutOfBoundsTileIdx(usize)
}