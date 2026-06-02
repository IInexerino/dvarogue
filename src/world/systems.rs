use std::collections::HashMap;
use bevy::{asset::AssetServer, ecs::system::{Commands, Res}, transform::components::Transform};
use crate::world::{floor::{CurrentFloor, DiscoveredFloors, DungeonFloor, DungeonKind}, map::{grid::{Map, grid_to_world_transform}, spatial::SpatialMap, tile::TilePos}};


pub fn setup_first_map(
    mut commands: Commands
) {
    let first_floor = DungeonFloor::first_floor(DungeonKind::Dungeon);
    let map = Map::new_from_dungeon_floor(first_floor).unwrap();

    commands.insert_resource(DiscoveredFloors(
        HashMap::from([(first_floor, (map, SpatialMap::new()))])
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
        let transform = Transform::from_translation(grid_to_world_transform(coords, 0.0)); 

        let sprite = tile.kind.to_sprite(&asset_server);

        commands.spawn((
            TilePos(coords),
            transform,
            sprite.expect("Tilekind with unconfigured sprite"),
        ));

    }
}