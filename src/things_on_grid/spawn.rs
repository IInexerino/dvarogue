use bevy::{asset::AssetServer, camera::Camera2d, ecs::{query::With, system::{Commands, Res, ResMut, Single}}, math::IVec2, sprite::Sprite, transform::components::Transform};
use crate::{input::keybinds::GameInput, menu::character_select::CharacterConfigs, things_on_grid::components::PlayerActorBundle, world::{floor::{CurrentFloor, DiscoveredFloors}, map::grid::grid_to_world_transform}};

/// System for building and spawning a player entity based on [`CharacterConfigs`].
/// 
/// To be executed when entering a game from the menu.
/// 
/// Removes [`CharacterConfigs`] when done.
pub fn spawn_starting_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_configs: Res<CharacterConfigs>,
    mut maps: ResMut<DiscoveredFloors>,
    current_floor: Res<CurrentFloor>,
    mut camera: Single<&mut Transform, With<Camera2d>>
) {
    let current_floor = maps.get_mut(&current_floor).expect("Error: CurrentFloor not present in DiscoveredFloors");
    let mapsize = &current_floor.0.size;
    let spatial_map = &mut current_floor.1;

    let pos = IVec2::new(
        mapsize.width / 2,
        mapsize.height / 2
    );

    let player_entity = commands.spawn(
        PlayerActorBundle::new(
            character_configs.starting_delay_mult, 
            pos, 
            character_configs.health, 
            character_configs.vision_radius,
            character_configs.background,
            Sprite::from_image(asset_server.load(&character_configs.sprite)),
        )
    ).id();

    camera.translation = grid_to_world_transform(pos, 2.0);

    spatial_map.push_to_position(pos, player_entity);

    commands.remove_resource::<CharacterConfigs>();
    commands.init_resource::<GameInput>();
}