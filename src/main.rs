// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::module_inception)]

use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};

pub mod components;
pub mod data;
pub mod resources;
pub mod systems;

mod hexachess_plugin;
use self::hexachess_plugin::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Hexachess"),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                // file_path: String::from("../assets"),
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins(HexachessPlugin);

    app.run();
}
