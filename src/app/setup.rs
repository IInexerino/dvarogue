use bevy::{camera::{Camera2d, OrthographicProjection, Projection, ScalingMode}, ecs::system::Commands, reflect::{FromReflect, TypeRegistry, serde::{ReflectDeserializer, ReflectSerializer}}};
use ron::ser::PrettyConfig;
use crate::input::keybinds::KeybindRegister;
use serde_core::de::DeserializeSeed;


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

    let mut type_registry = TypeRegistry::default(); 
    type_registry.register::<KeybindRegister>();
    // first time entering the game. sets up all data files that can be set to default 
    let data_folder_path = std::path::Path::new("data");
    if !data_folder_path.exists() {
        std::fs::create_dir("data").unwrap();
        
        let input = KeybindRegister::default();
        let reflect_serializer = ReflectSerializer::new(&input, &type_registry);
        let default_keybinds_string = ron::ser::to_string_pretty(&reflect_serializer, PrettyConfig::new()).unwrap();

        std::fs::write("data/keybinds.ron", default_keybinds_string).unwrap();
    }

    let keybinds_string = std::fs::read_to_string("data/keybinds.ron").unwrap();
    let reflect_deserializer = ReflectDeserializer::new(&type_registry);
    let deserialized_value = reflect_deserializer.deserialize(
    &mut ron::Deserializer::from_str(&keybinds_string).unwrap()
    ).unwrap();
    
let keybinds = <KeybindRegister as FromReflect>::from_reflect(&*deserialized_value).unwrap();
commands.insert_resource(keybinds);

}
