use bevy::{ecs::{resource::Resource, system::{Res, ResMut}}, input::{ButtonInput, keyboard::KeyCode}, platform::collections::HashSet, reflect::Reflect};

use crate::{input::Dir, settings::keybinds::SettingsKeybindRegister};


#[derive(Clone, Copy, Reflect, Eq, PartialEq, Hash)]
#[reflect(PartialEq, Hash)]
pub enum InputKind {
    ToggleZoom,
    Move(Dir)
}

#[derive(Resource, Default)]
pub struct GameInput {
    pressed: HashSet<InputKind>,
    just_pressed: HashSet<InputKind>,
    just_released: HashSet<InputKind>,
}

impl GameInput {
    pub fn pressed(&self, input: &InputKind) -> bool {
        self.pressed.contains(input)
    }

    pub fn just_pressed(&self, input: &InputKind) -> bool {
        self.just_pressed.contains(input)
    }

    pub fn just_released(&self, input: &InputKind) -> bool {
        self.just_released.contains(input)
    }

    pub fn just_pressed_or_pressed(&self, input: &InputKind) -> bool {
        self.just_pressed(input) || self.pressed(input)
    }

    pub fn get_just_pressed(&self) -> bevy::platform::collections::hash_set::Iter<'_, InputKind> {
        self.just_pressed.iter()
    }

    pub fn get_pressed(&self) -> bevy::platform::collections::hash_set::Iter<'_, InputKind>  {
        self.pressed.iter()
    }
}

pub fn update_game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    bindings: Res<SettingsKeybindRegister>,
    mut game_input: ResMut<GameInput>,
) {
    game_input.pressed.clear();
    game_input.just_pressed.clear();
    game_input.just_released.clear();

    for binding in &bindings.0 {
        let action = binding.input_kind;
        let key = binding.binding;

        if keyboard.pressed(key) {
            game_input.pressed.insert(action);
        }

        if keyboard.just_pressed(key) {
            game_input.just_pressed.insert(action);
        }

        if keyboard.just_released(key) {
            game_input.just_released.insert(action);
        }
    }
}