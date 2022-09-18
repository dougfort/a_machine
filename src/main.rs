use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod cli;
mod keyboard;
mod sprites;
mod tape;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<cli::Args>()
        .init_resource::<sprites::Sprites>()
        .init_resource::<tape::Tape>()
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(keyboard::keyboard_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn step(mut commands: Commands, sprites: Res<sprites::Sprites>, tape: Res<tape::Tape>) {
    const SPRITE_WIDTH: f32 = 32.0;

    tape.symbols.iter().enumerate().for_each(|(i, symbol)| {
        println!("{}: {:?}", i, symbol);
        let sprite = sprites.get(&symbol);
        commands
            .spawn_bundle(SpriteBundle {
                texture: sprite,
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 - (tape.symbols.len() as f32 / 2.0)) * SPRITE_WIDTH,
                    0.0,
                    0.0,
                )),
                ..Default::default()
            })
            .insert(tape::Symbol(symbol.clone()))
            .insert(tape::TapeIndex(i));
    });
}
