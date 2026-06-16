use std::fmt::Display;
use bevy::{color::palettes::css::{BLACK, GRAY, LIGHT_GRAY, ORANGE, RED, WHITE}, ecs::{component::Component, entity::{ContainsEntity, Entity}, observer::On, query::With, resource::Resource, system::{Commands, Query, ResMut, Single}}, picking::events::{Click, Out, Over, Pointer}, state::state::NextState, ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val}, utils::default};
use rand::seq::IndexedRandom;
use crate::{app::states::MainMenuState, things_on_grid::components::Health, ui::generic_widgets::generic_button};



/// Initial range of characters, analagous to 'species' in many games like DCSS.
/// 
/// to be done: 
///     - inclusion of diverse classes, making a prior struct that will contain this enum, 
///     and the class choice, From will be implemented for that type instead
#[derive(Component, Clone, Copy)]
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
    pub sprite_idx: usize
}

impl CharacterConfigs {
    fn new(max_hp: i32, starting_delay_mult: u64, vision_radius: u8, background: CharacterBackground, sprite_idx: usize) -> Self {
        Self {
            health: Health { hp: max_hp, max_hp },
            starting_delay_mult,
            vision_radius,
            background,
            sprite_idx,
        }
    }
}

// Different character builds derived directly from the different char selections
impl From<CharacterBackground> for CharacterConfigs {
    fn from(value: CharacterBackground) -> Self {
        match value {
            CharacterBackground::GreyOrb => {
                CharacterConfigs::new(10, 100, 8, value, 3)
            },
            CharacterBackground::Mamut => {
                CharacterConfigs::new(8, 120, 8, value, 4)
            },
            CharacterBackground::Furio => {
                CharacterConfigs::new(12, 75, 8, value, 2)
            },
        }
    }
}



#[derive(Component)]
pub struct MMCharSelectionScreen;

pub fn setup_mm_char_selection(mut commands: Commands) {
    commands.spawn((
        MMCharSelectionScreen,
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(WHITE.into()), 
    )).with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                width: Val::Percent(33.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(BLACK.into()), 
        )).with_children(|parent| {
            let button_hover_observer = | 
                hovering: On<Pointer<Over>>, 
                mut query: Query<&mut BackgroundColor>
            | {
                let mut bckg_color = query.get_mut(hovering.entity).unwrap();
                *bckg_color = BackgroundColor(LIGHT_GRAY.into());  
            };

            let button_out_observer = | 
                hovering: On<Pointer<Out>>, 
                mut query: Query<&mut BackgroundColor>
            | {
                let mut bckg_color = query.get_mut(hovering.entity).unwrap();
                *bckg_color = BackgroundColor(WHITE.into());  
            };
            
            parent.spawn(
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    ..default()
                }
            ).with_children(|parent| {
                parent.spawn( generic_button("The Gray Orb", GRAY, false))
                .observe(button_hover_observer).observe(button_out_observer)
                .observe(|_: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>>, mut commands: Commands | {
                    commands.insert_resource(CharacterConfigs::from(CharacterBackground::GreyOrb));
                    mm_state.set(MainMenuState::InMainMenu)
                });

                parent.spawn( generic_button("Furio the Furious", ORANGE, false))
                .observe(button_hover_observer).observe(button_out_observer)
                .observe(|_: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>>, mut commands: Commands | {
                    commands.insert_resource(CharacterConfigs::from(CharacterBackground::Furio));
                    mm_state.set(MainMenuState::InMainMenu)
                });

                parent.spawn( generic_button("Mamut", RED, false))
                .observe(button_hover_observer).observe(button_out_observer)
                .observe(|_: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>>, mut commands: Commands | {
                    commands.insert_resource(CharacterConfigs::from(CharacterBackground::Mamut));
                    mm_state.set(MainMenuState::InMainMenu)
                });
            });

            parent.spawn( generic_button("back to main menu", BLACK, true))
            .observe(button_hover_observer).observe(button_out_observer)
            .observe(|_: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>> | {
                mm_state.set(MainMenuState::InMainMenu)
            });
        });
    });
}

pub fn mm_char_selection_cleanup(mut commands: Commands, entity: Single<Entity, With<MMCharSelectionScreen>>) {
    commands.entity(entity.entity()).despawn();  
}