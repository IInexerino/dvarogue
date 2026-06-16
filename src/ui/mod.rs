pub mod hud;
pub mod mm_character_select;
pub mod generic_widgets;
pub mod main_menu;
pub mod settings_menu;

use bevy::{app::{Plugin, Update}, ecs::schedule::IntoScheduleConfigs, state::{app::AppExtStates, condition::in_state, state::{OnEnter, OnExit}}};

use crate::{app::states::{FloorState, MainMenuState}, ui::{hud::{update_topleft_ui, update_topright_ui}, main_menu::{main_menu_cleanup, setup_main_menu}, mm_character_select::{mm_char_selection_cleanup, setup_mm_char_selection}}};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .init_state::<MainMenuState>()
        
        .add_systems(OnEnter(MainMenuState::InMainMenu), setup_main_menu )
        .add_systems(OnExit(MainMenuState::InMainMenu), main_menu_cleanup )

        .add_systems(OnEnter(MainMenuState::CharSelectionMenu), setup_mm_char_selection)
        .add_systems(OnExit(MainMenuState::CharSelectionMenu), mm_char_selection_cleanup )
        ;
    }
}

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(Update, (
                    update_topright_ui,
                    update_topleft_ui,
                ).run_if(in_state(FloorState::InFloor))
            )
        ;
    }
}