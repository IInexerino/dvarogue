pub mod procedural_gen;

use thiserror::Error;
use std::{collections::HashMap};
use bevy::{ asset::AssetServer, color::{Color, palettes::css::BLACK}, ecs::{component::Component, entity::Entity, resource::Resource, system::{Commands, Res}}, math::IVec2, prelude::{Deref, DerefMut}, sprite::Sprite, transform::components::Transform};


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DungeonKind {
    Dungeon,
    Caves,
}

/// Specifies a dungeon kind and a specific floor
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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

#[derive(Clone, PartialEq)]
pub enum TileKind {
    Floor,
    WallBedrock,
    WallAlloy,
    WallRock,
    WallWood,
    WallDirt,
    ShallowWater,
    DeepWater,
    StairsDown,
    Door(bool),
}

impl TileKind {
    fn to_sprite(&self, asset_server: &Res<AssetServer>) -> Option<Sprite> {
        match self {
            TileKind::Floor => Some(Sprite::from_image(asset_server.load("purple_floor.png"))),
            TileKind::WallBedrock => Some(Sprite::from_image(asset_server.load("wallrock.png"))),
            TileKind::WallRock => Some(Sprite::from_image(asset_server.load("wallrock.png"))),
            TileKind::Door(false) => {
                let mut sprite = Sprite::from_image(asset_server.load("wallrock.png"));
                sprite.color = Color::Srgba(BLACK);
                Some(sprite)
            },
            TileKind::Door(true) => Some(Sprite::from_image(asset_server.load("purple_floor.png"))),

            _ => None
        }
    }
}

pub enum CollisionKind {
    None,
    Solid,
    Digable(u8),
    DeepWater,
}

impl From<&TileKind> for CollisionKind {
    fn from(value: &TileKind) -> Self {
        match value {
            TileKind::WallBedrock => Self::Solid,
            TileKind::Door(false) => Self::Solid,
            TileKind::WallDirt => Self::Digable(1),
            TileKind::WallWood => Self::Digable(2),
            TileKind::WallRock => Self::Digable(4),
            TileKind::WallAlloy => Self::Digable(8),
            TileKind::DeepWater => Self::DeepWater,
            _ => Self::None
        }
    }
}

pub fn setup_first_map(
    mut commands: Commands
) {
    let first_floor = DungeonFloor::first_floor(DungeonKind::Dungeon);

    let map = Map::new_from_dungeon_floor(&first_floor).unwrap();

    commands.insert_resource(DiscoveredFloors(
        HashMap::from([(first_floor.clone(), (map, SpatialMap::new_empty()))])
    ));
    commands.insert_resource(CurrentFloor(first_floor));
}

pub fn render_current_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_floor: Res<CurrentFloor>,
    discovered_floors: Res<DiscoveredFloors>
) {
    let (map, spatial_map) = &discovered_floors.0[&current_floor.0];

    for (idx, tile) in map.tiles.iter().enumerate() {
        let coords = map.idx_to_coords(idx);
        let transform = Transform::from_xyz(
            ((coords.x * 32) - (((map.size.width+1) * 32) / 2)) as f32   , 
            ((coords.y * 32) - (((map.size.height+1) * 32)) / 2) as f32 , 
            -1.
        ); 

        let sprite = tile.kind.to_sprite(&asset_server);

        commands.spawn((
            TilePos(coords),
            transform,
            sprite.expect("Tilekind with unconfigured sprite"),
        ));

    }
}

/// Errors related to this and any child modules
#[derive(Error, Debug)]
pub enum MapError {
    #[error("Floor '{0:?}' invalid" )]
    InvalidFloorSpecification(DungeonFloor),
    #[error("Tile idx {0} is out of bounds" )]
    OutOfBoundsTileIdx(usize)
}

/// Resource keeping track of the current floor the player is on
#[derive(Resource, Deref, DerefMut)]
pub struct CurrentFloor(pub DungeonFloor);

/// Resource storing all discovered and generated floors, including current
#[derive(Resource, Deref, DerefMut)]
pub struct DiscoveredFloors(HashMap<DungeonFloor, (Map, SpatialMap)>);

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

pub struct MapSize {
    pub width: i32,
    pub height: i32
}

impl MapSize {
    pub fn new(height: i32, width: i32) -> Self { MapSize {height, width} }

    pub fn count(&self) -> usize { (self.width * self.height) as usize }
}

#[derive(Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub visible: bool,
    pub discovered: bool,
}

impl From<TileKind> for Tile {
    fn from(value: TileKind) -> Self {
        Tile {kind: value, visible: false, discovered: false }
    }
}

#[derive(Resource)]
pub struct SpatialMap {
    pub entities: HashMap<IVec2, Vec<Entity>>,
}

impl SpatialMap {
    pub fn new_empty() -> Self {
        SpatialMap { entities: HashMap::new() }
    }
}

#[derive(Component)]
pub struct TilePos(pub IVec2);