mod game;
mod menu;

use bevy::{camera::ScalingMode, prelude::*};
use crate::{game::{actors::{CharacterConfigs, actions::execute_actions, spawn_starting_player}, inputs::{GameInput, InputKind, KeybindRegister, register_player_input, update_game_input}, map::{render_current_map, setup_first_map}, scheduler::{cycle_actions, reset_clock, reset_scheduler}, ui::{UiNeedsUpdate, init_game_ui, update_topleft_ui, update_topright_ui}}, menu::{choose_character, enter_game}};

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    
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
            ).run_if(resource_exists_and_equals(UiNeedsUpdate(true))),
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

    app.run();
}


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    InMainMenu,
    InLevel,
    BetweenLevels,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::InLevel)]
pub enum TurnState {
    #[default]
    CyclingActors,
    PerformingActions,
    AwaitingPlayerInput,

}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::InLevel)]
pub enum InPlayerMenu {
    #[default]
    InGame,
    InMenu,
}


// ----- Run Conditions -----


// ----- Systems -----

fn setup(mut commands: Commands) {
    // - spawning camera -
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.1,
            scaling_mode: ScalingMode::AutoMax { max_width: 960., max_height: 540. },
            ..OrthographicProjection::default_2d()
        })
    ));

    // setting up default keybinds or loading them from prev. 

    // first time entering the game. sets up all data files that can be set to default 
    if !std::path::Path::new("data").exists() {
        std::fs::create_dir("data").unwrap();

        let default_keybinds_string = serde_json::to_string(&KeybindRegister::default()).unwrap();
        std::fs::write("data/keybinds.json", default_keybinds_string).unwrap();
    }

    let keybinds_string = std::fs::read_to_string("data/keybinds.json").unwrap();
    let keybinds: KeybindRegister = serde_json::from_str(&keybinds_string).unwrap();
    commands.insert_resource(keybinds);

}

fn toggle_zoom(
    mut projection: Single<&mut Projection, With<Camera2d>>,
    game_inputs: Res<GameInput>,
) {
    if game_inputs.just_pressed(&InputKind::ToggleZoom ) {
        if let Projection::Orthographic(ortho ) = projection.as_mut() { 
            match ortho.scale {
                1.1 => ortho.scale = 0.75,
                0.75 => ortho.scale = 1.1,
                _ => panic!("Error: ortho.scale had an invalid value: {}", ortho.scale)
            }
        } else { panic!("Error: Camera2d projection was not orthographic") }
    }
}