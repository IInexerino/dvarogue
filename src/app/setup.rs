use bevy::{camera::{Camera2d, OrthographicProjection, Projection, ScalingMode}, ecs::system::Commands};
use crate::input::keybinds::KeybindRegister;


pub fn setup(mut commands: Commands) {
    // - spawning camera -
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.1,
            scaling_mode: ScalingMode::AutoMax { max_width: 960., max_height: 540. },
            ..OrthographicProjection::default_2d()
        })
    ));

    // setting up default keybinds or loading them from prev. 

    // first time entering the game. sets up all data files that can be set to default 
    if !std::path::Path::new("data").exists() {
        std::fs::create_dir("data").unwrap();

        let default_keybinds_string = serde_json::to_string(&KeybindRegister::default()).unwrap();
        std::fs::write("data/keybinds.json", default_keybinds_string).unwrap();
    }

    let keybinds_string = std::fs::read_to_string("data/keybinds.json").unwrap();
    let keybinds: KeybindRegister = serde_json::from_str(&keybinds_string).unwrap();
    commands.insert_resource(keybinds);

}
