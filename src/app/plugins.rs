use bevy::{app::{Plugin, Startup, Update}, ecs::schedule::{IntoScheduleConfigs, common_conditions::{resource_exists, not}}, state::{app::AppExtStates, condition::in_state, state::{OnEnter, OnExit}}};

use crate::{action::execute::execute_actions, app::{setup::setup, states::{GameState, InPlayerMenu, TurnState}}, input::{keybinds::update_game_input, misc_inputs::toggle_zoom, player_turn_input::register_player_input}, menu::{character_select::{CharacterConfigs, choose_character}, systems::enter_game}, things_on_grid::spawn::spawn_starting_player, turn::{clock::reset_clock, scheduler::{cycle_actions, reset_scheduler}}, ui::hud::{init_game_ui, update_topleft_ui, update_topright_ui}, world::systems::{render_current_map, setup_first_map}};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .init_state::<GameState>()
            .add_sub_state::<TurnState>()
            .add_sub_state::<InPlayerMenu>();

        // startup systems
        app
            .add_systems(Startup, setup)

        // new game startup systems
            .add_systems(OnExit(GameState::InMainMenu),(
                (
                    setup_first_map,
                    spawn_starting_player,
                    reset_clock,
                    init_game_ui,
                ).chain(),
            )
        )
        
        // new level startup systems
        .add_systems(OnEnter(GameState::InLevel), (
            reset_scheduler, 
            render_current_map,
        ))

        .add_systems(Update,(
            // menu update systems
            (
                choose_character.run_if(not(resource_exists::<CharacterConfigs>)),
                // exit menu into game system
                enter_game.run_if(resource_exists::<CharacterConfigs>)
            ).run_if(in_state(GameState::InMainMenu)),

            (
                (
                    update_topright_ui,
                    update_topleft_ui,
                ),
                (
                    update_game_input,
                    (
                        (
                            cycle_actions,
                        ).run_if(in_state(TurnState::CyclingActors)),
                        (
                            execute_actions,
                        ).run_if(in_state(TurnState::PerformingActions)),
                        (
                            (
                                register_player_input
                            ).run_if(in_state(TurnState::AwaitingPlayerInput)),
                            toggle_zoom
                        ).run_if(in_state(InPlayerMenu::InGame))
                    ),
                ).chain()
            ).run_if(in_state(GameState::InLevel))
        ));
    }
}