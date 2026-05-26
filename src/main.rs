mod game;
mod menu;

use bevy::{camera::ScalingMode, prelude::*};
use crate::{game::{actors::{CharacterConfigs, spawn_starting_player}, map::{render_current_map, setup_first_map}, scheduler::{reset_clock, reset_scheduler}, ui::{UiNeedsUpdate, init_game_ui, update_topleft_ui, update_topright_ui}}, menu::{choose_character, enter_game}};


fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest()
    ));
    
    app.init_state::<GameState>();

    // startup systems
    app.add_systems(Startup, setup);

    // new game startup systems
    app.add_systems(OnExit(GameState::InMenu),(
            (
                setup_first_map,
                spawn_starting_player,
                reset_clock,
                init_game_ui,
            ).chain(),
        )
    );
    
    // new level startup systems
    app.add_systems(OnEnter(GameState::InLevel), (
        reset_scheduler, 
        render_current_map,
    ));

    app.add_systems(Update,(
        // menu update systems
        (
            choose_character.run_if(not(resource_exists::<CharacterConfigs>)),
            // exit menu into game system
            enter_game.run_if(resource_exists::<CharacterConfigs>)
        ).run_if(in_state(GameState::InMenu)),
        (
            // ui update systems
            (
                update_topright_ui,
                update_topleft_ui,
            ).run_if(resource_exists_and_equals(UiNeedsUpdate(true))),
        ).run_if(in_state(GameState::InLevel))
    ));

    app.run();
}


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    InMenu,
    InLevel,
    BetweenLevels,
}


// ----- Run Conditions -----


// ----- Systems -----

fn setup(mut commands: Commands) {
    // - spawning camera -
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 3.5,
            scaling_mode: ScalingMode::AutoMax { max_width: 960., max_height: 540. },
            ..OrthographicProjection::default_2d()
        })
    ));
}