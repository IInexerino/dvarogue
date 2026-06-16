use bevy::{color::{Srgba, palettes::css::WHITE}, ecs::{bundle::Bundle, children}, text::{TextColor, TextFont}, ui::{AlignItems, BackgroundColor, JustifyContent, Node, UiRect, Val, widget::{Button, Text}}, utils::default};

pub fn generic_button(button_text: &str, text_color: Srgba, save: bool) -> impl Bundle {
    (
        Node {
            height: Val::Px(50.0),
            width: Val::Percent(90.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: 
                if !save { UiRect::top(Val::Px(15.0)) } 
                else { 
                    UiRect { 
                        top: Val::Auto, 
                        bottom: Val::Px(15.0), 
                        ..default() 
                    }
                },
            ..default()
        },
        Button,
        BackgroundColor(WHITE.into()), 
        children![(
            Text::new(button_text),
            TextColor(text_color.into()),
            TextFont { 
                font_size: 20.0,
                ..default()
            },
        )],
    )
}