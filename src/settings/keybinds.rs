
use bevy::{ ecs::resource::Resource, input::keyboard::KeyCode::{self, *}, reflect::{FromReflect, Reflect, TypeRegistry, serde::{ReflectDeserializer, ReflectSerializer}}};
use ron::ser::PrettyConfig;
use serde_core::de::DeserializeSeed;
use crate::{action::kinds::RotationalDir, input::{Dir, centralization::InputKind}};

#[derive(Resource, Reflect, PartialEq)]
pub struct SettingsKeybindRegister(pub Vec<InputBinding>);

impl SettingsKeybindRegister {
    pub fn load_or_default() -> Self {
        let mut type_registry = TypeRegistry::default(); 
        type_registry.register::<SettingsKeybindRegister>();
        
        let keybinds_path = std::path::Path::new("data/keybinds.ron");

        if keybinds_path.exists() {
            let keybinds_string = std::fs::read_to_string(keybinds_path).unwrap();
            let reflect_deserializer = ReflectDeserializer::new(&type_registry);
            
            let deserialized_value = reflect_deserializer.deserialize(
                &mut ron::Deserializer::from_str(&keybinds_string).unwrap()
            ).unwrap();

            <SettingsKeybindRegister as FromReflect>::from_reflect(&*deserialized_value).unwrap()
        } else {
            // first time entering the game. sets up all data files that can be set to default 
            if !std::path::Path::new("data").exists() {
                std::fs::create_dir("data").unwrap();
            }
            
            let input = SettingsKeybindRegister::default();
            let reflect_serializer = ReflectSerializer::new(&input, &type_registry);
            let default_keybinds_string = ron::ser::to_string_pretty(&reflect_serializer, PrettyConfig::new()).unwrap();

            std::fs::write(keybinds_path, default_keybinds_string).unwrap();
            input
        }
    }

    fn save(&self) {
        let mut type_registry = TypeRegistry::default(); 
        type_registry.register::<SettingsKeybindRegister>();

        let reflect_serializer = ReflectSerializer::new(self, &type_registry);
        let keybinds_string = ron::ser::to_string_pretty(&reflect_serializer, PrettyConfig::new()).unwrap();

        if !std::path::Path::new("data").exists() {
            std::fs::create_dir("data").unwrap();
        }
            
        std::fs::write("data/keybinds.ron", keybinds_string).unwrap();
    }
}

impl Default for SettingsKeybindRegister {
    fn default() -> Self {
        SettingsKeybindRegister(Vec::from([
            InputBinding::new(InputKind::ToggleZoom, KeyZ),
            InputBinding::new(InputKind::Move(Dir::W), ArrowLeft),
            InputBinding::new(InputKind::Move(Dir::E), ArrowRight),
            InputBinding::new(InputKind::Move(Dir::N), ArrowUp),
            InputBinding::new(InputKind::Move(Dir::S), ArrowDown),
            InputBinding::new(InputKind::Move(Dir::W), Numpad4),
            InputBinding::new(InputKind::Move(Dir::E), Numpad6),
            InputBinding::new(InputKind::Move(Dir::N), Numpad8),
            InputBinding::new(InputKind::Move(Dir::S), Numpad2),
            InputBinding::new(InputKind::Move(Dir::NW), Numpad7),
            InputBinding::new(InputKind::Move(Dir::NE), Numpad9),
            InputBinding::new(InputKind::Move(Dir::SE), Numpad3),
            InputBinding::new(InputKind::Move(Dir::SW), Numpad1),
            InputBinding::new(InputKind::Move(Dir::SW), Numpad1),
            InputBinding::new(InputKind::Wait, Numpad5),
            InputBinding::new(InputKind::Wait, Period),
            InputBinding::new(InputKind::Rotate(RotationalDir::Clockwise), KeyT),
            InputBinding::new(InputKind::Rotate(RotationalDir::CounterClockwise), KeyR),
            InputBinding::new(InputKind::PickupItems, KeyE),
        ]))
    }
}

#[derive(Reflect, Eq, PartialEq, Hash)]
#[reflect(PartialEq, Hash)]
pub struct InputBinding {
    pub input_kind: InputKind,
    pub binding: KeyCode,
}

impl InputBinding {
    pub fn new(input_kind: InputKind, binding: KeyCode) -> Self {
        Self { input_kind, binding }
    }
}
