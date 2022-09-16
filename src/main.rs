use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;

mod cli;
mod keyboard;

enum Alphabet {
    Blank,
    Zero,
    One,
}

struct Tape(Vec<Alphabet>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<cli::Args>()
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(Tape(vec![]))
        .add_startup_system(setup)
        .add_system(keyboard::keyboard_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn step(mut commands: Commands, asset_server: Res<AssetServer>) {
    const SPRITE_WIDTH: f32 = 32.0;
    const SPRITE_COUNT: usize = 10;
    let blank_sprite: Handle<Image> = asset_server.load("blank.png");
    let zero_sprite: Handle<Image> = asset_server.load("zero.png");
    let one_sprite: Handle<Image> = asset_server.load("one.png");
    let mut rng = rand::thread_rng();

    for i in 0..SPRITE_COUNT {
        let sprite = match rng.gen_range(0..3) {
            0 => zero_sprite.clone(),
            1 => one_sprite.clone(),
            _ => blank_sprite.clone(),
        };
        commands.spawn_bundle(SpriteBundle {
            texture: sprite,
            transform: Transform::from_translation(Vec3::new(
                (i as f32 - (SPRITE_COUNT as f32 / 2.0)) * SPRITE_WIDTH,
                0.0,
                0.0,
            )),
            ..Default::default()
        });
    }
}
