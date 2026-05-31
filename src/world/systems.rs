use std::collections::HashMap;

use bevy::{asset::AssetServer, ecs::system::{Commands, Res}, transform::components::Transform};

use crate::world::{floor::{CurrentFloor, DiscoveredFloors, DungeonFloor, DungeonKind}, map::{grid::Map, spatial::SpatialMap, tile::TilePos}};


pub fn setup_first_map(
    mut commands: Commands
) {
    let first_floor = DungeonFloor::first_floor(DungeonKind::Dungeon);

    let map = Map::new_from_dungeon_floor(&first_floor).unwrap();
    let mapsize = map.size.clone();

    commands.insert_resource(DiscoveredFloors(
        HashMap::from([(first_floor.clone(), (map, SpatialMap::new_empty(&mapsize)))])
    ));
    commands.insert_resource(CurrentFloor(first_floor));
}

pub fn render_current_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_floor: Res<CurrentFloor>,
    discovered_floors: Res<DiscoveredFloors>
) {
    let (map, _ ) = &discovered_floors.0[&current_floor.0];

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