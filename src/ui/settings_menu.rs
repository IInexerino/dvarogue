use bevy::{color::palettes::css::{BLACK, LIGHT_GRAY, WHITE}, ecs::{component::Component, observer::On, system::{Commands, Query}}, picking::events::{Out, Over, Pointer}, ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val}, utils::default};

use crate::ui::generic_widgets::generic_button;


#[derive(Component)]
pub struct MMSettingsScreen;

pub fn setup_mm_char_selection(mut commands: Commands) {
    commands.spawn((
        MMSettingsScreen,
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
                width: Val::Percent(66.0),
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