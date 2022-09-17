use bevy::prelude::*;

use crate::cli;
use crate::sprites;
use crate::step;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    args: Res<cli::Args>,
    mut commands: Commands,
    sprites: Res<sprites::Sprites>,
) {
    if args.step && keys.just_pressed(KeyCode::Space) {
        step(commands, sprites);
    }
}
