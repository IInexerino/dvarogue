use std::fmt::Display;

use bevy::ecs::component::Component;

#[derive(Component, Copy, Clone)]
pub struct Health {
    pub hp: i32,
    pub max_hp: i32,
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.hp, self.max_hp)
    }
}