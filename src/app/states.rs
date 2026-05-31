use bevy::state::state::{States, SubStates, StateSet};

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