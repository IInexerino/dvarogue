mod app;

mod input;
mod things_on_grid;
mod turn;
mod world;
mod ui;
mod action;
mod settings;

use bevy::{DefaultPlugins, app::{App, PluginGroup}, image::ImagePlugin, utils::default, window::{PresentMode, Window, WindowPlugin}};
use crate::app::plugins::GamePlugin;

fn main() {
    let mut app = App::new();
    
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
        GamePlugin
    ));

    app.run();
}