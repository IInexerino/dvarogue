use bevy::{ecs::entity::Entity, math::IVec2, reflect::Reflect};


pub enum Action {
    Move(IVec2),
    Rotate(RotationalDir),
    /// With attacked Entity id
    Attack (Entity),
    Wait,
    /// With door Entity id
    OpenDoor (IVec2),
    PickupItems
}

impl Action {
    pub fn to_delay(&self) -> u64 {
        match self {
            Action::Move(_) => 100,
            Action::Rotate(_) => 25,
            Action::Attack(_) => 100,
            Action::Wait => 100,
            Action::OpenDoor(_) => 50,
            Action::PickupItems => 150,
        }
    }

    pub fn to_delay_with_multiplier(&self, mult: u64) -> u64 {
        (self.to_delay() * mult) / 100
    }
}

#[derive(Clone, Copy, Reflect, Eq, PartialEq, Hash)]
pub enum RotationalDir {
    Clockwise,
    CounterClockwise,
}