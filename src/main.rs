use bevy::prelude::*;

use bevy_inspector_egui::WorldInspectorPlugin;

mod cli;
mod keyboard;
mod rules;
mod sprites;
mod state;
mod tape;

const WINDOW_WIDTH: f32 = 1000.;
const WINDOW_HEIGHT: f32 = 600.;

const SPRITE_WIDTH: f32 = 32.0;
const SPRITE_COUNT: usize = (WINDOW_WIDTH / SPRITE_WIDTH) as usize - 1;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Automatic Machine".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<cli::Args>()
        .init_resource::<sprites::SpriteArray>()
        .init_resource::<tape::Tape>()
        .init_resource::<rules::RuleSet>()
        .insert_resource(rules::StepCount(0))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(keyboard::keyboard_input)
        .add_system(rules::step)
        .run();
}

fn setup(mut commands: Commands, args: Res<cli::Args>, sprite_array: Res<sprites::SpriteArray>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(state::State(args.initial_state.clone()));

    const STARTING_SYMBOL: &str = " ";
    for i in 0..SPRITE_COUNT {
        let sprite = sprite_array.get(STARTING_SYMBOL);
        commands
            .spawn_bundle(SpriteBundle {
                texture: sprite,
                visibility: Visibility { is_visible: false },
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 - (SPRITE_COUNT as f32 / 2.0)) * SPRITE_WIDTH,
                    0.0,
                    0.0,
                )),
                ..Default::default()
            })
            .insert(tape::TapeIndex(i))
            .insert(sprites::SpriteSymbol(STARTING_SYMBOL.to_string()));
    }
}
