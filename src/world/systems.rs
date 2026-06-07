use bevy::{ecs::system::{Commands, Res}, transform::components::Transform};
use crate::{app::setup::SpriteSheet, world::{floor::{CurrentFloor, DiscoveredFloors}, map::{grid::grid_to_world_transform, tile::TilePos}}};

pub fn render_current_map(
    mut commands: Commands,
    spritesheet: Res<SpriteSheet>,
    current_floor: Res<CurrentFloor>,
    discovered_floors: Res<DiscoveredFloors>
) {
    let (map, _ ) = &discovered_floors.0[&current_floor.0];

    for (idx, tile) in map.tiles.iter().enumerate() {
        let coords = map.idx_to_coords(idx);
        let transform = Transform::from_translation(grid_to_world_transform(coords, 0.0)); 

        let sprite = tile.kind.to_sprite(
                spritesheet.sprite.clone(), 
                spritesheet.texture_atlas_layout.clone()
        ).expect("Tilekind with unconfigured sprite");

        commands.spawn((
            TilePos(coords),
            transform,
            sprite,
        ));

    }
}