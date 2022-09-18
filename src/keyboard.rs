use bevy::prelude::*;

use crate::cli;
use crate::sprites;
use crate::tape;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    args: Res<cli::Args>,
    commands: Commands,
    sprites: Res<sprites::Sprites>,
    tape: Res<tape::Tape>
) {
    if args.step && keys.just_pressed(KeyCode::Space) {
        println!("Space");
    }
}
