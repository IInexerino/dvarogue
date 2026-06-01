use bevy::reflect::Reflect;

pub mod keybinds;
pub mod player_turn_input;
pub mod misc_inputs;

#[derive(Clone, Copy, PartialEq, Reflect, Eq, Hash)]
#[reflect(PartialEq, Hash)]
pub enum Dir {
    N,
    S,
    E,
    W
}