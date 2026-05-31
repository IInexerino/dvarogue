use std::fmt::Display;
use bevy::{ecs::{component::Component, resource::Resource, system::{Commands, Res}}, input::{ButtonInput, keyboard::KeyCode}};
use rand::seq::IndexedRandom;
use crate::things_on_grid::components::Health;



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
    pub starting_delay_mult: u64,
    pub vision_radius: u8,
    pub background: CharacterBackground,
    pub sprite: String
}

impl CharacterConfigs {
    fn new(max_hp: i32, starting_delay_mult: u64, vision_radius: u8, background: CharacterBackground, sprite: String) -> Self {
        Self {
            health: Health { hp: max_hp, max_hp },
            starting_delay_mult,
            vision_radius,
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
                CharacterConfigs::new(10, 100, 8, value, "grey_orb.png".to_string())
            },
            CharacterBackground::Mamut => {
                CharacterConfigs::new(8, 120, 8, value, "mamut.png".to_string())
            },
            CharacterBackground::Furio => {
                CharacterConfigs::new(12, 75, 8, value, "furio.png".to_string())
            },
        }
    }
}



pub fn choose_character(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands
) {
    let character_configs: CharacterConfigs = if keys.just_pressed(KeyCode::KeyO) {
        CharacterBackground::GreyOrb.into()
    } else if keys.just_pressed(KeyCode::KeyM) {
        CharacterBackground::Mamut.into()
    } else if keys.just_pressed(KeyCode::KeyF) {
        CharacterBackground::Furio.into()
    } else {
        return
    };

    commands.insert_resource(character_configs);
}