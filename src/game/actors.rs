pub mod combat;
pub mod actions;

use std::{fmt::Display, thread::current};
use bevy::{asset::AssetServer, ecs::{component::Component, resource::Resource, system::{Commands, Res, ResMut}}, math::IVec2, prelude::{Deref, DerefMut}, reflect::Reflect, sprite::Sprite};
use rand::seq::IndexedRandom;
use crate::game::{actors::{actions::Action, combat::Health}, inputs::GameInput, map::{CurrentFloor, DiscoveredFloors}};

/// System for building and spawning a player entity based on [`CharacterConfigs`].
/// 
/// To be executed when entering a game from the menu.
/// 
/// Removes [`CharacterConfigs`] when done.
pub fn spawn_starting_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_configs: Res<CharacterConfigs>,
    mut maps: ResMut<DiscoveredFloors>,
    current_floor: Res<CurrentFloor>
) {
    let current_floor = maps.get_mut(&current_floor).expect("Error: CurrentFloor not present in DiscoveredFloors");
    let mapsize = &current_floor.0.size;
    let spatial_map = &mut current_floor.1;

    let coords = IVec2::new(
        mapsize.width / 2 + 1,
        mapsize.height / 2 + 1
    );

    let player_entity = commands.spawn(
        (
            PlayerActor,
            Actor{ delay_mult: character_configs.starting_delay_multiplier },
            character_configs.health,
            Sprite::from_image(asset_server.load(&character_configs.sprite)),
            character_configs.background.clone(),
            Position(coords)
        )
    ).id();

    println!("{:?}", coords);

    spatial_map.entities.get_mut(&coords).expect("Error: Coords not present in SpatialMap").push(player_entity);

    commands.remove_resource::<CharacterConfigs>();
    commands.insert_resource(PlayerSelectedAction(None));
    commands.init_resource::<GameInput>();
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
#[derive(Component)]
pub struct Actor {
    pub delay_mult: u64
}

impl Actor {
    
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

/// Marker [`Component`] for enemy actor entities.
/// 
/// automatically adds default components: [`Actor`]
#[derive(Component)]
#[require(SelectedAction)]
pub struct EnemyActor;

#[derive(Component, Default, Deref, DerefMut)]
pub struct SelectedAction(pub Option<Action>);


#[derive(Deref, DerefMut, Resource)]
pub struct PlayerSelectedAction(pub Option<Action>);


/// Marker [`Component`] for the player's character.
/// 
/// automatically adds default components: [`Actor`], [`Position`]
#[derive(Component)]
#[require(Position)]
pub struct PlayerActor;

/// [`Component`] for the player character Entity, defining the player's visibility radius.
#[derive(Component)]
pub struct Vision {
    pub radius: u8
}

/// [`Component`] for the game grid position of an Entity.
#[derive(Component, Default, Deref, DerefMut)]
pub struct Position(pub IVec2);

