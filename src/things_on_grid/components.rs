use std::fmt::Display;
use bevy::{ecs::{bundle::Bundle, component::Component}, math::IVec2, prelude::{Deref, DerefMut}, sprite::Sprite, transform::components::Transform};

use crate::{action::kinds::Action, main_menu::character_select::CharacterBackground, world::map::grid::grid_to_world_transform};


/// Marker [`Component`] for enemy actor entities.
/// 
/// automatically adds default components: [`Actor`]
#[derive(Component)]
#[require(Actor)]
pub struct EnemyActor;

#[derive(Bundle)]
pub struct EnemyActorBundle {
    enemy_actor: EnemyActor,
    actor: Actor,
    pub pending_action: PendingAction,
    pub position: Position,
    pub health: Health,
    pub sprite: Sprite
}

impl EnemyActorBundle {
    pub fn new(delay_mult: u64, pos: IVec2, health: Health, sprite: Sprite) -> Self {
        EnemyActorBundle {
            enemy_actor: EnemyActor,
            actor: Actor,
            pending_action: PendingAction::new(delay_mult),
            position: Position(pos),
            health,
            sprite
        }
    }
}

/// Marker [`Component`] for the player's character.
/// 
/// automatically adds default components: [`Actor`], [`Position`]
#[derive(Component)]
pub struct PlayerActor;

#[derive(Bundle)]
pub struct PlayerActorBundle {
    player_actor: PlayerActor,
    actor: Actor,
    pub pending_action: PendingAction,
    pub position: Position,
    pub health: Health,
    pub vision: Vision,
    pub background: CharacterBackground,
    pub sprite: Sprite,
    pub transform: Transform

}

impl PlayerActorBundle {
    pub fn new(delay_mult: u64, pos: IVec2, health: Health, vision_radius: u8, background: CharacterBackground, sprite: Sprite,) -> Self {
        PlayerActorBundle {
            player_actor: PlayerActor,
            actor: Actor,
            pending_action: PendingAction::new(delay_mult),
            position: Position(pos),
            health,
            vision: Vision{ radius: vision_radius},
            background,
            sprite,
            transform: Transform::from_translation(grid_to_world_transform(pos, 1.0))

        }
    }
}


/// Marker [`Component`] for any Entity that would be able to engage in actions.
/// 
/// automatically adds default components: [`PendingAction`]
#[derive(Component, Default)]
pub struct Actor;

#[derive(Component)]
pub struct PendingAction {
    pub action: Option<Action>,
    pub delay_mult: u64
}

impl PendingAction {
    pub fn new(delay_mult: u64) -> Self {
        PendingAction { 
            action: None, 
            delay_mult 
        }
    }
    
    /// Displays `self.delay` as divided by 100, with two decimal points, without using floats.
    /// 
    /// This is speficially to format it for presentation in the top-right-ui.
    /// 
    /// Examples: 0 = "0.00", 175 = "1.75"
    pub fn to_decimal_string(&self) -> String {
        let delay = Action::Wait.to_delay_with_multiplier(self.delay_mult);

        let s = delay.to_string();
        // Possible Bug? ; Could this possibly return more or less 
        let len = s.len();

        return if len == 1 {
            format!("0.0{}", s)
        } else if len == 2 {
            format!("0.{}", s) 
        } else {
            format!("{}.{}", &s[..len-2], &s[len-2..])
        }
    }
}


/// [`Component`] for the player character Entity, defining the player's visibility radius.
#[derive(Component)]
pub struct Vision {
    pub radius: u8
}

/// [`Component`] for the game grid position of an Entity.
#[derive(Component, Default, Deref, DerefMut)]
pub struct Position(pub IVec2);

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