use bevy::{ecs::system::{Commands, Res, ResMut}, input::{ButtonInput, keyboard::KeyCode}, state::state::NextState};
use crate::{GameState, game::actors::{CharacterBackground, CharacterConfigs}};


pub fn enter_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InLevel);
    }
}

pub fn choose_character(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands
) {
    let character_configs: CharacterConfigs = if keys.just_pressed(KeyCode::KeyO) {
        CharacterBackground::GreyOrb.into()
    } else if keys.just_pressed(KeyCode::KeyM) {
        CharacterBackground::Mamut.into()
    } else if keys.just_pressed(KeyCode::KeyF) {
        CharacterBackground::Furio.into()
    } else {
        return
    };

    commands.insert_resource(character_configs);
}