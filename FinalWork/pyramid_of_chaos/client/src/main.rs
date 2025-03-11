use avian3d::PhysicsPlugins;
use bevy::{ app::App, log::LogPlugin, prelude::*, winit::{ UpdateMode, WinitSettings } };
use client::core::game_default::GameCore;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(60)),
        })
        .insert_resource(ClearColor(Color::srgb(102.0 / 255.0, 153.0 / 255.0, 204.0 / 255.0)))
        .add_plugins(GameCore)
        .run();
}
