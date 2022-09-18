use bevy::prelude::*;

use crate::sprites;
use crate::tape;

pub struct RefreshTimer(pub Timer);

pub fn refresh(mut commands: Commands, 
    mut refresh_timer: ResMut<RefreshTimer>,
    time: Res<Time>,
    sprites: Res<sprites::Sprites>, 
    tape: Res<tape::Tape>,
    query: Query<(Entity, &tape::TapeIndex)>,
) {
    const SPRITE_WIDTH: f32 = 32.0;

    refresh_timer.0.tick(time.delta());
    if !refresh_timer.0.finished() {
        return;
    }

    for (entity, tape_index) in query.iter() {
        commands.entity(entity).despawn();
    }

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
