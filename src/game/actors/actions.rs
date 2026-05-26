use bevy::{ecs::entity::Entity, math::IVec2};

pub enum Action {
    Move(IVec2),
    /// With attacked Entity id
    Attack (Entity),
    Wait,
    /// With door Entity id
    OpenDoor (IVec2),
    Pickup (Entity)
}

impl Action {
    pub fn to_delay(&self) -> u64 {
        match self {
            Action::Move(_) => 100,
            Action::Attack(_) => 100,
            Action::Wait => 100,
            Action::OpenDoor(_) => 50,
            Action::Pickup(_) => 150,
        }
    }
}