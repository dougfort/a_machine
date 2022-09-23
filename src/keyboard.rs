use bevy::prelude::*;

use crate::cli;
use crate::rules;
use crate::sprites;
use crate::state;
use crate::tape;

pub fn keyboard_input(
    commands: Commands,
    args: Res<cli::Args>,
    keys: Res<Input<KeyCode>>,
    rule_set: Res<rules::RuleSet>,
    state: ResMut<state::State>,
    mut tape: ResMut<tape::Tape>,
    sprite_array: Res<sprites::SpriteArray>,
    query: Query<(Entity, &tape::TapeIndex)>,
) {
    if args.step && keys.just_pressed(KeyCode::Space) {
        println!("Step");
        rules::step(commands, rule_set, state, tape, sprite_array, query);
        return;
    }
    if keys.just_pressed(KeyCode::Delete) {
        println!("Delete");
        tape.set(" ");
    }
    if keys.just_pressed(KeyCode::Key0) {
        println!("0");
        tape.set("0'.to_string());");
    }
    if keys.just_pressed(KeyCode::Key1) {
        println!("1");
        tape.set("1");
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
