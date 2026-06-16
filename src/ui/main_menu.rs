use bevy::{app::AppExit, color::palettes::css::{BLACK, DARK_GREY, LIGHT_GRAY, WHITE}, ecs::{component::Component, entity::{ContainsEntity, Entity}, message::MessageWriter, observer::On, query::With, system::{Commands, Query, Res, ResMut, Single}}, picking::events::{Click, Out, Over, Pointer}, state::state::NextState, ui::{AlignItems, BackgroundColor, FlexDirection, Node, Val}, utils::default};

use crate::{app::states::MainMenuState, ui::{generic_widgets::generic_button, mm_character_select::CharacterConfigs}};


#[derive(Component)]
pub struct MainMenuScreen;


pub fn setup_main_menu(
    mut commands: Commands,
    char_configs: Option<Res<CharacterConfigs>>,
) {
    commands.spawn((
        MainMenuScreen,
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(WHITE.into()), 
    )).with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                width: Val::Percent(20.0),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BLACK.into()),
        )).with_children(|parent| {

            let button_hover_observer = 
            | hovering: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor> | {
                let mut bckg_color = query.get_mut(hovering.entity).unwrap();
                *bckg_color = BackgroundColor(LIGHT_GRAY.into());  
            };

            let button_out_observer = 
            | hovering: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor> | {
                let mut bckg_color = query.get_mut(hovering.entity).unwrap();
                *bckg_color = BackgroundColor(WHITE.into());  
            };

            parent.spawn(generic_button(
                "enter game", 
                if char_configs.is_none() { DARK_GREY } 
                    else { BLACK }, false 
            ))
            .observe(button_hover_observer).observe(button_out_observer)
            .observe(|_: On<Pointer<Click>>, mut main_menu_state: ResMut<NextState<MainMenuState>>, char_configs: Option<Res<CharacterConfigs>> | {
                    if char_configs.is_some() { main_menu_state.set(MainMenuState::InGame); }
            });

            parent.spawn(generic_button("select character", BLACK, false))
            .observe(button_hover_observer).observe(button_out_observer)
            .observe(| _: On<Pointer<Click>>, mut mm_state: ResMut<NextState<MainMenuState>> | {
                mm_state.set(MainMenuState::CharSelectionMenu);
            });
            
            parent.spawn(generic_button("settings", BLACK, false))
            .observe(button_hover_observer).observe(button_out_observer);

            parent.spawn(generic_button("save and exit", BLACK, true))
            .observe(button_hover_observer).observe(button_out_observer)
            .observe(| _: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success);
            });
        });
    });
}

pub fn main_menu_cleanup(mut commands: Commands, entity: Single<Entity, With<MainMenuScreen>>) {
    commands.entity(entity.entity()).despawn();  
}