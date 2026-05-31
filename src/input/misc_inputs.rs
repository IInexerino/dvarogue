use bevy::{camera::{Camera2d, Projection}, ecs::{query::With, system::{Res, Single}}};
use crate::input::keybinds::{GameInput, InputKind};


pub fn toggle_zoom(
    mut projection: Single<&mut Projection, With<Camera2d>>,
    game_inputs: Res<GameInput>,
) {
    if game_inputs.just_pressed(&InputKind::ToggleZoom ) {
        if let Projection::Orthographic(ortho ) = projection.as_mut() { 
            match ortho.scale {
                1.1 => ortho.scale = 0.75,
                0.75 => ortho.scale = 1.1,
                _ => panic!("Error: ortho.scale had an invalid value: {}", ortho.scale)
            }
        } else { panic!("Error: Camera2d projection was not orthographic") }
    }
}