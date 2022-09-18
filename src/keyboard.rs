use bevy::prelude::*;

use crate::cli;
use crate::sprites;
use crate::tape;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    args: Res<cli::Args>,
    commands: Commands,
    sprites: Res<sprites::Sprites>,
    mut tape: ResMut<tape::Tape>
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("Space");
        tape.set(tape::Alphabet::Blank);
    }
    if keys.just_pressed(KeyCode::Key0) {
        println!("0");
        tape.set(tape::Alphabet::Zero);
    }
    if keys.just_pressed(KeyCode::Key1) {
        println!("1");
        tape.set(tape::Alphabet::One);
    }
    if keys.just_pressed(KeyCode::Left) {
        println!("Left");
        tape.move_left();
    }
    if keys.just_pressed(KeyCode::Right) {
        println!("Right");
        tape.move_right();
    }
}
