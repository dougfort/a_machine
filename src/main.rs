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
const SPRITE_HEIGHT: f32 = 32.0;
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

pub fn center_sprite_index() -> usize {
    SPRITE_COUNT / 2
}

fn setup(
    mut commands: Commands,
    args: Res<cli::Args>,
    sprite_array: Res<sprites::SpriteArray>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(state::State(args.initial_state.clone()));

    const STARTING_SYMBOL: &str = " ";
    let entities = (0..SPRITE_COUNT).map(|i| {
        let x = (i as f32 - center_sprite_index() as f32) * SPRITE_WIDTH;
        println!("setup x: {}", x);
        let sprite = sprite_array.get(STARTING_SYMBOL);
        let id = commands
            .spawn_bundle(SpriteBundle {
                texture: sprite,
                visibility: Visibility { is_visible: false },
                transform: Transform::from_translation(Vec3::new(x, 0.0, 0.0)),
                ..Default::default()
            })
            .insert(tape::TapeIndex(i))
            .insert(sprites::SpriteSymbol(STARTING_SYMBOL.to_string()))
            .id();
        (id, x)
    });
    let entity_vec: Vec<(Entity, f32)> = entities.collect();

    let (_, x) = entity_vec[center_sprite_index()];
    let y = -SPRITE_HEIGHT;
    let head_arrow_sprite: Handle<Image> = asset_server.load("up_arrow.png");
    commands.spawn_bundle(SpriteBundle {
        texture: head_arrow_sprite,
        visibility: Visibility { is_visible: true },
        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
        ..Default::default()
    });

    commands.insert_resource(rules::EntityArray(entity_vec));
}
