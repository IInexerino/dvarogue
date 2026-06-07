use bevy::{ecs::{component::Component, query::With, system::{ Res, Single}}, ui::widget::Text};
use crate::{things_on_grid::components::{Health, PendingAction, PlayerActor}, turn::clock::Clock};


// ----- Topleft Ui -----

/// Marker [`Component`] for the top-left Ui node.
#[derive(Component)]
pub struct TopLeftUi;

/// System that makes a query for data related to top-left ui, and writes this data to the displayed [`Text`].
/// 
/// Should run on schedule `Update` while in game, and come with a run condition where [`UiNeedsUpdate`].0 == true. 
pub fn update_topleft_ui(
    player_data: Single<(&Health, &PendingAction), With<PlayerActor>>,
    mut topleft_ui: Single<&mut Text, With<TopLeftUi>>,
) {
    let char_bcg = topleft_ui.0.lines().next().unwrap();

    topleft_ui.0 = format!(
        "{}\nhealth: {}\ndelay*: {}", 
            char_bcg,
            player_data.0.to_string(),
            player_data.1.to_decimal_string(), 
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
