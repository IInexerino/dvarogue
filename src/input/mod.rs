use bevy::{math::IVec2, reflect::Reflect};

pub mod player_turn_input;
pub mod misc_inputs;
pub mod centralization;

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Eq, Hash)]
#[reflect(PartialEq, Hash)]
pub enum Dir {
    NW = 0,
    N = 1,
    NE = 2,
    E = 3,
    SE = 4,
    S = 5,
    SW = 6,
    W = 7,
}

impl Dir {
    pub fn into_delta_offset(&self) -> IVec2 {
        match self {
            Dir::N => IVec2::new(0, 1),
            Dir::S => IVec2::new(0, -1),
            Dir::E => IVec2::new(1, 0),
            Dir::W => IVec2::new(-1, 0),
            Dir::NE => IVec2::new(1, 1),
            Dir::NW => IVec2::new(-1, 1),
            Dir::SW => IVec2::new(-1, -1),
            Dir::SE => IVec2::new(1, -1),
        }
    }
    pub fn from_usize(num: usize) -> Option<Self> {
        match num {
            0 => Some(Dir::NW),
            1 => Some(Dir::N),
            2 => Some(Dir::NE),
            3 => Some(Dir::E),
            4 => Some(Dir::SE),
            5 => Some(Dir::S),
            6 => Some(Dir::SW),
            7 => Some(Dir::W),
            _ => None
        }
    }
}