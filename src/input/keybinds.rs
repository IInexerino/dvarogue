use std::collections::HashSet;

use bevy::{ecs::{resource::Resource, system::{Res, ResMut}}, input::{ButtonInput, keyboard::KeyCode}};
use serde::{Deserialize, Serialize};

use crate::input::Dir;


const CHAR_KEY_MAP: [(char, KeyCode); 38] = [
    ('z', KeyCode::KeyZ),
    ('x', KeyCode::KeyX),
    ('c', KeyCode::KeyC),
    ('v', KeyCode::KeyV),
    ('b', KeyCode::KeyB),
    ('n', KeyCode::KeyN),
    ('m', KeyCode::KeyM),
    (',', KeyCode::Comma),
    ('.', KeyCode::Period),
    ('/', KeyCode::Slash),
    ('a', KeyCode::KeyA),
    ('s', KeyCode::KeyS),
    ('d', KeyCode::KeyD),
    ('f', KeyCode::KeyF),
    ('g', KeyCode::KeyG),
    ('h', KeyCode::KeyH),
    ('j', KeyCode::KeyJ),
    ('k', KeyCode::KeyK),
    ('l', KeyCode::KeyL),
    (';', KeyCode::Semicolon),
    ('\'', KeyCode::Quote),
    ('\\', KeyCode::Backslash),
    ('q', KeyCode::KeyQ),
    ('w', KeyCode::KeyW),
    ('e', KeyCode::KeyE),
    ('r', KeyCode::KeyR),
    ('t', KeyCode::KeyT),
    ('y', KeyCode::KeyY),
    ('u', KeyCode::KeyU),
    ('i', KeyCode::KeyI),
    ('o', KeyCode::KeyO),
    ('p', KeyCode::KeyP),
    ('[', KeyCode::BracketLeft),
    (']', KeyCode::BracketRight),
    ('←', KeyCode::ArrowLeft),
    ('→', KeyCode::ArrowRight),
    ('↑', KeyCode::ArrowUp),
    ('↓', KeyCode::ArrowDown),
];


pub fn kb_key_to_char(input: KeyCode) -> Option<char> {
    for (char, keycode) in CHAR_KEY_MAP {
        if keycode == input {
            return Some(char)
        }
    }
    None
}

#[derive(Resource, Serialize, Deserialize)]
pub struct KeybindRegister(pub HashSet<InputBinding>);

impl KeybindRegister {
    pub fn get_bound_key(&self, input_kind: InputKind) -> KeyCode {
        let game_input = self.0.iter().find(|&s| s.input_kind == input_kind).expect("Error: InputKind is not registered");
        game_input.kb_char_to_key()
    } 
}

impl Default for KeybindRegister {
    fn default() -> Self {
        KeybindRegister(HashSet::from([
            InputBinding::new(InputKind::ToggleZoom, 'z'),
            InputBinding::new(InputKind::Move(Dir::W), '←'),
            InputBinding::new(InputKind::Move(Dir::E), '→'),
            InputBinding::new(InputKind::Move(Dir::N), '↑'),
            InputBinding::new(InputKind::Move(Dir::S), '↓'),
        ]))
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct InputBinding {
    input_kind: InputKind,
    binding: char,
}

impl InputBinding {
    pub fn new(input_kind: InputKind, binding: char) -> Self {
        Self { input_kind, binding }
    }
        
    pub fn kb_char_to_key(&self) -> KeyCode {
        CHAR_KEY_MAP.iter().find(| (s, _ )| s == &self.binding).expect("Error: character does not have a KeyCode mapping").1
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
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

    pub fn get_just_pressed(&self) -> std::collections::hash_set::Iter<'_, InputKind> {
        self.just_pressed.iter()
    }

    pub fn get_pressed(&self) -> std::collections::hash_set::Iter<'_, InputKind> {
        self.pressed.iter()
    }
}

pub fn update_game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    bindings: Res<KeybindRegister>,
    mut game_input: ResMut<GameInput>,
) {
    game_input.pressed.clear();
    game_input.just_pressed.clear();
    game_input.just_released.clear();

    for binding in &bindings.0 {
        let action = binding.input_kind.clone();
        let key = binding.kb_char_to_key();

        if keyboard.pressed(key) {
            game_input.pressed.insert(action.clone());
        }

        if keyboard.just_pressed(key) {
            game_input.just_pressed.insert(action.clone());
        }

        if keyboard.just_released(key) {
            game_input.just_released.insert(action);
        }
    }
}