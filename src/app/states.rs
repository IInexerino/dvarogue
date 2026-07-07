use bevy::state::state::{States, SubStates, StateSet};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainMenuState {
    #[default]
    None,
    InMainMenu,
    CharSelectionMenu,
    InGame
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloorState {
    #[default]
    None,
    InFloor,
    BetweenFloors
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(FloorState = FloorState::InFloor)]
pub enum TurnState {
    #[default]
    CyclingActors,
    PerformingActions,
    AwaitingPlayerInput,

}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(FloorState = FloorState::InFloor)]
pub enum IngameMenuState {
    #[default]
    None,
    Settings,
}