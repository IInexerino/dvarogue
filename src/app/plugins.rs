use bevy::{app::{Plugin, Startup, Update}, ecs::schedule::{IntoScheduleConfigs, common_conditions::{resource_exists, not}}, state::{app::AppExtStates, condition::in_state, state::OnEnter}};

use crate::{action::execute::execute_actions, app::{setup::{setup, setup_game}, states::{FloorState, IngameMenuState, MainMenuState, TurnState}}, input::{ centralization::update_game_input, misc_inputs::toggle_zoom, player_turn_input::register_player_input}, main_menu::{character_select::{CharacterConfigs, choose_character}, systems::enter_game}, turn::scheduler::{cycle_actions, reset_scheduler}, ui::hud::{ update_topleft_ui, update_topright_ui}, world::systems::render_current_map};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .init_state::<MainMenuState>()
            .init_state::<FloorState>()
            .add_sub_state::<TurnState>()
            .add_sub_state::<IngameMenuState>();

        // startup systems
        app
            .add_systems(Startup, 
                setup
            )

        // new game startup systems
            .add_systems(OnEnter(MainMenuState::InGame),
            setup_game,
        )
        
        // new level startup systems
        .add_systems(OnEnter(FloorState::InFloor), (
            reset_scheduler, 
            render_current_map,
        ))

        .add_systems(Update,(
            // menu update systems
            (
                choose_character.run_if(not(resource_exists::<CharacterConfigs>)),
                // exit menu into game system
                enter_game.run_if(resource_exists::<CharacterConfigs>)
            ).run_if(in_state(MainMenuState::InMainMenu)),

            (
                (
                    update_topright_ui,
                    update_topleft_ui,
                ),
                (
                    update_game_input,
                    (
                        cycle_actions.run_if(in_state(TurnState::CyclingActors)),
                        execute_actions.run_if(in_state(TurnState::PerformingActions)),
                        (
                            register_player_input.run_if(in_state(TurnState::AwaitingPlayerInput)),
                            toggle_zoom
                        ).run_if(in_state(IngameMenuState::None))
                    ),
                )
            ).run_if(in_state(FloorState::InFloor))
        ));
    }
}