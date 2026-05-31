use bevy::math::IVec2;

use crate::world::{MapError, floor::{DungeonFloor, DungeonKind}, map::tile::{Tile, TileKind}};


/// Main Resource storing `element by element - row by row` grid tile data in indexable order.
/// 
/// Methods containing configurations for generating the next floor.
/// 
/// For the moment it only contains a set map size derived from each level. 
/// Later that will be slightly randomized, and it will contain several 
/// other specifications for procedural generation  that may be randomized somewhat such as:
/// - whether diagonal movement and squares that could only be accessed by it are allowed
/// - to what extent
/// 
/// *For the random room placement, perfect maze, remove dead ends, place doors procedural gen technique*
/// - the room density (via attempts)
/// - the amount of dead ends removed
/// 
/// *Other procgen techniques*
/// - cellular automata levels
pub struct Map {
    pub size: MapSize,
    pub tiles: Vec<Tile>,
    pub regions:Vec<Vec<IVec2>>,
}

impl Map {
    pub fn new_filled(size: MapSize) -> Self {
        Map {
            tiles: vec![Tile::from(TileKind::WallRock); size.count()],
            size,
            regions: Vec::new(),
        }
    }

    /// The function actually configuring a floor from configs. Dungeon procgen occurs here.
    pub fn new_from_dungeon_floor(floor: &DungeonFloor) -> Result<Self, MapError> {
        let size = match floor.kind {
            DungeonKind::Dungeon => {
                match floor.floor {
                    1..=2 => MapSize::new(49, 75),
                    3..=5 => MapSize::new(59, 99),
                    6..=10 => MapSize::new(39, 59),
                    _ => return Err(MapError::InvalidFloorSpecification(floor.clone()))
                }
            },
            _ => return Err(MapError::InvalidFloorSpecification(floor.clone())),
        };

        let mut map = Map::new_filled(size);
        
        map.set_boundary_border();
        map.place_random_rooms(150);
        map.draw_perfect_mazes();
        map.unify_regions();
        map.remove_dead_ends(500);

        Ok(map)
    }

    /// Returns index corresponding to the tile positioned on the coordinates. 
    /// 
    /// Can be used to index self.tiles for that tile.
    pub fn coord_to_idx(&self, coords: &IVec2) -> usize {
        (coords.y * self.size.width + coords.x) as usize
    }
    /// Returns coordinates corresponding to a specified index. 
    /// 
    /// Useful when enumerating self.tiles
    pub fn idx_to_coords(&self, idx: usize) -> IVec2 {
        let idx = idx as i32;
        IVec2 { 
            x: idx % self.size.width, 
            y: idx / self.size.width
        }
    }

    /// Gets a `&Tile`, or returns an out-of-bounds `MapError` 
    /// that specifies what index value went out of bounds
    pub fn get_tile(&self, coords: &IVec2) -> Result<&Tile, MapError> {
        let idx = self.coord_to_idx(coords); 
        self.tiles.get(idx).ok_or(MapError::OutOfBoundsTileIdx(idx))
    }
    /// Only to be used in map generation, as using Tile::from() yields an undiscoverd and out-of-view tile
    pub fn set_tile(&mut self, coords: &IVec2, kind: TileKind) -> Result<(), MapError> {
        let idx = self.coord_to_idx(coords);
        return 
            if idx >= self.size.count() { Err(MapError::OutOfBoundsTileIdx(idx)) }
            else { self.tiles[idx] = Tile::from(kind); Ok(()) }
    }
}

#[derive(Clone)]
pub struct MapSize {
    pub width: i32,
    pub height: i32
}

impl MapSize {
    pub fn new(height: i32, width: i32) -> Self { MapSize {height, width} }

    pub fn count(&self) -> usize { (self.width * self.height) as usize }
}