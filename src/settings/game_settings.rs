use bevy::{ecs::resource::Resource, window::{MonitorSelection, VideoModeSelection, WindowMode, WindowResolution}};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

// resources will be initialized, other things are either parts of resources, or purely for serialization


#[derive(Default, Serialize, Deserialize)]
pub struct GameSettings {
    pub language: SettingsLanguage,
    pub window: SettingsWindow,
}

impl GameSettings {
    pub fn load_or_default() -> Self {
        let settings_path = std::path::Path::new("data/settings.ron");

        if settings_path.exists() {
            let settings_string = std::fs::read_to_string(settings_path).unwrap();
            let settings_struct: GameSettings = ron::from_str(&settings_string).unwrap();
            settings_struct
        } else {
            let default_settings = GameSettings::default();
            default_settings.save();
            default_settings
        }
    }

    pub fn save(&self) {
        if !std::path::Path::new("data").exists() {
            std::fs::create_dir("data").unwrap();
        }
        
        let settings_string = ron::ser::to_string_pretty(self, PrettyConfig::default()).unwrap();
        std::fs::write("data/settings.ron", settings_string).unwrap();
    }

}

#[derive(Resource, Default, Serialize, Deserialize)]
pub enum SettingsLanguage {
    #[default]
    English,
    Russian
}

#[derive(Default, Resource, Serialize, Deserialize)]
pub struct SettingsWindow {
    pub window_resolution: SettingsWindowResolution,
    pub fullscreen: SettingsWindowMode,
    pub monitor_selection: Option<usize>,
}

#[derive(Default, Serialize, Deserialize)]
pub enum SettingsWindowResolution {
    #[default]
    R1280x720,
    R1600x900,
    R1920x1080,
    R2560x1440,
    R3440x1440,
    R3840x2160,
}

impl From<SettingsWindowResolution> for WindowResolution {
    fn from(value: SettingsWindowResolution) -> Self {
        match value {
            SettingsWindowResolution::R1280x720 => WindowResolution::new(1280, 720),
            SettingsWindowResolution::R1600x900 => WindowResolution::new(1600, 900),
            SettingsWindowResolution::R1920x1080 => WindowResolution::new(1920, 1080),
            SettingsWindowResolution::R2560x1440 => WindowResolution::new(2560, 1440),
            SettingsWindowResolution::R3440x1440 => WindowResolution::new(3440, 1440),
            SettingsWindowResolution::R3840x2160 => WindowResolution::new(3840, 2160),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub enum SettingsWindowMode {
    Fullscreen(usize),
    BorderlessFullscreen(usize),
    #[default]
    Windowed,
}

impl From<SettingsWindowMode> for WindowMode {
    fn from(value: SettingsWindowMode) -> Self {
        match value {
            SettingsWindowMode::Fullscreen(monitor) => WindowMode::Fullscreen(MonitorSelection::Index(monitor), VideoModeSelection::Current),
            SettingsWindowMode::BorderlessFullscreen(monitor) => WindowMode::BorderlessFullscreen(MonitorSelection::Index(monitor)),
            SettingsWindowMode::Windowed => WindowMode::Windowed,
        }
    }
}