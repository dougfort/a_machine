use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;

mod cli;
mod keyboard;
mod sprites;

#[derive(Clone)]
enum Alphabet {
    Blank,
    Zero,
    One,
}

#[derive(Component, Clone)]
struct Symbol(Alphabet);
#[derive(Component)]
struct TapeIndex(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<cli::Args>()
        .init_resource::<sprites::Sprites>()
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(keyboard::keyboard_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn step(mut commands: Commands, sprites: Res<sprites::Sprites>) {
    const SPRITE_WIDTH: f32 = 32.0;
    const SPRITE_COUNT: usize = 10;
    let mut rng = rand::thread_rng();

    for i in 0..SPRITE_COUNT {
        let symbol = match rng.gen_range(0..2) {
            0 => Alphabet::Zero,
            1 => Alphabet::One,
            _ => Alphabet::Blank,
        };
        let sprite = sprites.get(&symbol);
        commands
            .spawn_bundle(SpriteBundle {
                texture: sprite,
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 - (SPRITE_COUNT as f32 / 2.0)) * SPRITE_WIDTH,
                    0.0,
                    0.0,
                )),
                ..Default::default()
            })
            .insert(Symbol(symbol))
            .insert(TapeIndex(i));
    }
}
