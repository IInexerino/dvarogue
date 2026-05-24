pub mod combat;
pub mod actions;

use std::fmt::Display;
use bevy::{asset::AssetServer, ecs::{component::Component, resource::Resource, system::{Commands, Res}}, math::IVec2, sprite::Sprite};
use rand::seq::IndexedRandom;
use crate::game::{actors::{actions::PendingAction, combat::Health}, scheduler::Timing};

/// System for building and spawning a player entity based on [`CharacterConfigs`].
/// 
/// To be executed when entering a game from the menu.
/// 
/// Removes [`CharacterConfigs`] when done.
pub fn spawn_starting_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_configs: Res<CharacterConfigs>
) {

    commands.spawn(
        (
            PlayerActor,
            character_configs.health,
            Timing {
                next_action_tick: 0,
                delay_multiplier: character_configs.starting_delay_multiplier,
            },
            Sprite::from_image(asset_server.load(&character_configs.sprite)),
            character_configs.background.clone()
        )
    );

    commands.remove_resource::<CharacterConfigs>();
}


/// Initial range of characters, analagous to 'species' in many games like DCSS.
/// 
/// to be done: 
///     - inclusion of diverse classes, making a prior struct that will contain this enum, 
///     and the class choice, From will be implemented for that type instead
#[derive(Component, Clone)]
pub enum CharacterBackground {
    GreyOrb,
    Mamut,
    Furio,
}

impl Display for CharacterBackground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = [
            "the Furious",
            "the Spiteful",
            "a Reflection of the Abyss",
            "a Victim of Brainrot",
            "the Critic",
            "the Critical Critic",
            "the Critic of Critical Criticism",
            "the Neurotypical",
            "in Posession of Moulaga",
            "an American Patriot",
            "the Lonely Afghani Jew",
            "an Overly Social American Jew",
            "a Crazy Bird Lady",
        ];
        let mut rng = rand::rng();

        let c = *r.choose(&mut rng).unwrap();
        return match self {
            CharacterBackground::GreyOrb => write!(f, "A Grey Orb, {}", c),
            CharacterBackground::Mamut => write!(f, "Mamut, {}", c),
            CharacterBackground::Furio => write!(f, "Furio the Furious"),
        };
    }
}

// ----- menu stuff -----

/// Resource that gives game startup systems information about initial player build.
/// It is configured and created while in the main menu.
/// 
/// Only after it has been created will the system that allows one to exit menu into the game be able to run.
#[derive(Resource)]
pub struct CharacterConfigs {
    pub health: Health,
    /// percentage multiplier (100 = 100%)
    pub starting_delay_multiplier: u64,
    pub background: CharacterBackground,
    pub sprite: String
}

impl CharacterConfigs {
    fn new(max_hp: i32, starting_delay_multiplier: u64, background: CharacterBackground, sprite: String) -> Self {
        Self {
            health: Health { hp: max_hp, max_hp },
            starting_delay_multiplier,
            background,
            sprite
        }
    }
}

// Different character builds derived directly from the different char selections
impl From<CharacterBackground> for CharacterConfigs {
    fn from(value: CharacterBackground) -> Self {
        match value {
            CharacterBackground::GreyOrb => {
                CharacterConfigs::new(10, 100, value, "grey_orb.png".to_string())
            },
            CharacterBackground::Mamut => {
                CharacterConfigs::new(8, 120, value, "mamut.png".to_string())
            },
            CharacterBackground::Furio => {
                CharacterConfigs::new(12, 75, value, "furio.png".to_string())
            },
        }
    }
}

/// Marker [`Component`] for any Entity that would be able to engage in actions.
/// 
/// automatically adds default components: [`PendingAction`]
#[derive(Component, Default)]
#[require(PendingAction)]
pub struct Actor;

/// Marker [`Component`] for enemy actor entities.
/// 
/// automatically adds default components: [`Actor`]
#[derive(Component)]
#[require(Actor)]
pub struct EnemyActor;

/// Marker [`Component`] for the player's character.
/// 
/// automatically adds default components: [`Actor`], [`Position`]
#[derive(Component)]
#[require(Actor, Position)]
pub struct PlayerActor;

/// Marker [`Component`] for the player character Entity, defining the player's visibility radius.
#[derive(Component)]
pub struct Visibility {
    pub radius: u8
}

/// Marker [`Component`] for the game grid position of an Entity.
#[derive(Component, Default)]
pub struct Position(pub IVec2);

