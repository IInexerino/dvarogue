use bevy::{ecs::system::{Res, ResMut}, input::{ButtonInput, keyboard::KeyCode}, state::state::NextState};
use crate::app::states::{FloorState, MainMenuState};

pub fn enter_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        main_menu_state.set(MainMenuState::InGame);
    }
}