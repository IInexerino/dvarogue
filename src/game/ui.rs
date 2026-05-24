use bevy::{ecs::{component::Component, query::With, resource::Resource, system::{Commands, Res, Single}}, ui::{Node, PositionType, px, widget::Text}, utils::default};
use crate::game::{actors::{CharacterBackground, PlayerActor, combat::Health}, scheduler::{Clock, Timing}};

/// Resource keeping track of whether any dynamic data displayed by the Ui has been changed. 
/// It determines the result of the run condition that is applied to the Ui Update systems. 
/// This avoids updating the Ui on every frame.
#[derive(Resource, Default, PartialEq)]
pub struct UiNeedsUpdate(pub bool);

// ----- Ui Initialization ----- 

/// System that spawns one top-right, and one top left ui bundle, and initializes the [`UiNeedsUpdate`] resource.
pub fn init_game_ui(
    mut commands: Commands,
    player_data: Single<(&CharacterBackground, &Health, &Timing), With<PlayerActor>>
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
        Text::new(format!(
            "{}\nhealth: {}\ndelay*: {}", 
                player_data.0.to_string(),
                player_data.1.to_string(), 
                player_data.2.to_decimal_string(), 
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

    commands.init_resource::<UiNeedsUpdate>();
}


// ----- Topleft Ui -----

/// Marker [`Component`] for the top-left Ui node.
#[derive(Component)]
pub struct TopLeftUi;

/// System that makes a query for data related to top-left ui, and writes this data to the displayed [`Text`].
/// 
/// Should run on schedule `Update` while in game, and come with a run condition where [`UiNeedsUpdate`].0 == true. 
pub fn update_topleft_ui(
    player_data: Single<(&Health, &Timing), With<PlayerActor>>,
    mut topleft_ui: Single<&mut Text, With<TopRightUi>>,
) {
    topleft_ui.0 = format!(
            "health: {}\ndelay mult: {}", player_data.0.to_string(), player_data.1.to_decimal_string(), 
    );
}

// ----- Topright Ui -----

/// Marker [`Component`] for the top-right Ui node.
#[derive(Component)]
pub struct TopRightUi;

/// System that makes a query for data related to top-right ui, and writes this data to the displayed [`Text`].
/// 
/// Should run on schedule `Update` while in game, and come with a run condition where [`UiNeedsUpdate`].0 == true. 
pub fn update_topright_ui(
    clock: Res<Clock>,
    mut topright_ui: Single<&mut Text, With<TopRightUi>>,
) {
    topright_ui.0 = format!(
        "Clock: {}", clock.to_decimal_string(), 
    );
}
