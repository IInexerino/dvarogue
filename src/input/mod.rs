use bevy::reflect::Reflect;

pub mod player_turn_input;
pub mod misc_inputs;
pub mod centralization;

#[derive(Clone, Copy, PartialEq, Reflect, Eq, Hash)]
#[reflect(PartialEq, Hash)]
pub enum Dir {
    N,
    S,
    E,
    W
}