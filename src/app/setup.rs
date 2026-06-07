use std::collections::HashMap;

use bevy::{asset::{AssetServer, Assets, Handle}, camera::{Camera2d, OrthographicProjection, Projection, ScalingMode}, ecs::{query::With, resource::Resource, system::{Commands, Res, ResMut, Single}}, image::{Image, TextureAtlas, TextureAtlasLayout}, math::{IVec2, UVec2}, render::view::Msaa, sprite::Sprite, state::state::NextState, transform::components::Transform, ui::{Node, PositionType, px, widget::Text}, utils::default};

use crate::{app::states::FloorState, input::centralization::GameInput, main_menu::character_select::CharacterConfigs, settings::{game_settings::GameSettings, keybinds::SettingsKeybindRegister}, things_on_grid::components::{PendingAction, PlayerActorBundle}, turn::clock::Clock, ui::hud::{TopLeftUi, TopRightUi}, world::{floor::{CurrentFloor, DiscoveredFloors, DungeonFloor, DungeonKind}, map::{grid::grid_to_world_transform, spatial::SpatialMap}}};

#[derive(Resource)]
pub struct SpriteSheet {
    pub sprite: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>
}

pub fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // - spawning camera -
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.1,
            scaling_mode: ScalingMode::AutoMax { max_width: 960., max_height: 540. },
            ..OrthographicProjection::default_2d()
        }),
        Msaa::Off,
    ));

    let texture_atlas_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::splat(32), 5, 1, None, None)
    );

    commands.insert_resource(SpriteSheet {
        sprite: asset_server.load("spritesheet_0.png"),
        texture_atlas_layout
    });

    // setting up default settings and keybinds or loading them from prev.
    let game_settings = GameSettings::load_or_default();
    commands.insert_resource(game_settings.language);
    commands.insert_resource(game_settings.window);

    commands.insert_resource(SettingsKeybindRegister::load_or_default());
}

// game setup

pub fn setup_game(
    mut commands: Commands,
    spritesheet: Res<SpriteSheet>,
    character_configs: Res<CharacterConfigs>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
    mut floor_state: ResMut<NextState<FloorState>>,
) { 
    let first_floor = DungeonFloor::first_floor(DungeonKind::Dungeon);
    let map = crate::world::map::grid::Map::new_from_dungeon_floor(first_floor).unwrap();

    let pos = IVec2::new(
        map.size.width / 2,
        map.size.height / 2
    );

    camera.translation = grid_to_world_transform(pos, 2.0);

    let player_entity = commands.spawn(
        PlayerActorBundle::new(
            character_configs.starting_delay_mult, 
            pos, 
            character_configs.health, 
            character_configs.vision_radius,
            character_configs.background,
            Sprite::from_atlas_image(
                spritesheet.sprite.clone(), 
                TextureAtlas {
                    layout: spritesheet.texture_atlas_layout.clone(),
                    index: character_configs.sprite_idx,
                }
            )
        )
    ).id();

    let mut spatial_map = SpatialMap::new();
    spatial_map.push_to_position(pos, player_entity);

    commands.insert_resource(DiscoveredFloors(
        HashMap::from([(first_floor, (map, spatial_map))])
    ));
    commands.insert_resource(CurrentFloor(first_floor));

    
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
        Text::new(format!(
            "{}\nhealth: {}\ndelay*: {}", 
                character_configs.background.to_string(),
                character_configs.health.to_string(), 
                PendingAction::new(character_configs.starting_delay_mult).to_decimal_string(), 
        )),
        TopLeftUi,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(12),
            ..default()
        },
        Text::new("Clock: 0.0"),
        TopRightUi,
    ));

    commands.insert_resource(Clock::default());
    commands.init_resource::<GameInput>();

    commands.remove_resource::<CharacterConfigs>();

    floor_state.set(FloorState::InFloor);
}


// floor setup