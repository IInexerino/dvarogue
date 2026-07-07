use std::fmt::Display;
use bevy::{color::palettes::css::{BLACK, GRAY, LIGHT_GRAY, ORANGE, RED, WHITE}, ecs::{component::Component, entity::{ContainsEntity, Entity}, observer::On, query::With, resource::Resource, system::{Commands, Query, ResMut, Single}}, picking::events::{Click, Out, Over, Pointer}, state::state::NextState, ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val}, utils::default};
use crate::{app::states::MainMenuState, ui::generic_widgets::generic_button};



/// Initial range of characters, analagous to 'species' in many games like DCSS.
/// 
/// to be done: 
///     - inclusion of diverse classes, making a prior struct that will contain this enum, 
///     and the class choice, From will be implemented for that type instead
#[derive(Component, Clone, Copy)]
pub enum CharacterBackground {
    Deserter,
    Saboteur,
    Smuggler,
}

impl Display for CharacterBackground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            CharacterBackground::Deserter => write!(f, "A Deserter"),
            CharacterBackground::Saboteur => write!(f, "A Saboteur"),
            CharacterBackground::Smuggler => write!(f, "A Smuggler"),
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
    pub vision_radius: u8,
    pub background: CharacterBackground,
    pub sprite_idx: usize
}

impl CharacterConfigs {
    fn new(vision_radius: u8, background: CharacterBackground, sprite_idx: usize) -> Self {
        Self {
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
            CharacterBackground::Deserter => {
                CharacterConfigs::new(8, value, 2)
            },
            CharacterBackground::Saboteur => {
                CharacterConfigs::new(8, value, 2)
            },
            CharacterBackground::Smuggler => {
                CharacterConfigs::new(8, value, 2)
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
                parent.spawn( generic_button("Deserter", GRAY, false))
                .observe(button_hover_observer).observe(button_out_observer)
                .observe(|_: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>>, mut commands: Commands | {
                    commands.insert_resource(CharacterConfigs::from(CharacterBackground::Deserter));
                    mm_state.set(MainMenuState::InMainMenu)
                });

                parent.spawn( generic_button("Saboteur", ORANGE, false))
                .observe(button_hover_observer).observe(button_out_observer)
                .observe(|_: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>>, mut commands: Commands | {
                    commands.insert_resource(CharacterConfigs::from(CharacterBackground::Saboteur));
                    mm_state.set(MainMenuState::InMainMenu)
                });

                parent.spawn( generic_button("Smuggler", RED, false))
                .observe(button_hover_observer).observe(button_out_observer)
                .observe(|_: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>>, mut commands: Commands | {
                    commands.insert_resource(CharacterConfigs::from(CharacterBackground::Smuggler));
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