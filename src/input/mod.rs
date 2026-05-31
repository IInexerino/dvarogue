use serde::{Deserialize, Serialize};

pub mod keybinds;
pub mod player_turn_input;
pub mod misc_inputs;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum Dir {
    N,
    S,
    E,
    W
}