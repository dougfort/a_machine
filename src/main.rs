use bevy::prelude::*;
use std::time::Duration;

use bevy_inspector_egui::WorldInspectorPlugin;

mod cli;
mod keyboard;
mod sprites;
mod state;
mod tape;
mod refresher;
mod rules;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<cli::Args>()
        .init_resource::<sprites::Sprites>()
        .init_resource::<tape::Tape>()
        .insert_resource(state::State("b".to_string()))
        .init_resource::<rules::RuleSet>()
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(keyboard::keyboard_input)
        .add_system(refresher::refresh)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(refresher::RefreshTimer(Timer::new(Duration::from_secs(1), true)));

}
