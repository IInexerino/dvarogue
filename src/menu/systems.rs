use bevy::{ecs::system::{Res, ResMut}, input::{ButtonInput, keyboard::KeyCode}, state::state::NextState};
use crate::app::states::GameState;

pub fn enter_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InLevel);
    }
}