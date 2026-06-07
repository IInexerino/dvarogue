mod app;

mod main_menu;
mod input;
mod things_on_grid;
mod turn;
mod world;
mod ui;
mod action;
mod settings;

use bevy::{DefaultPlugins, app::{App, PluginGroup}, image::ImagePlugin};
use crate::app::plugins::GamePlugin;

fn main() {
    let mut app = App::new();
    
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        GamePlugin
    ));

    app.run();
}