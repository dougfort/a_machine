use bevy::prelude::*;

use crate::cli;
use crate::rules;

pub fn keyboard_input(
    args: Res<cli::Args>,
    keys: Res<Input<KeyCode>>,
    mut step_count: ResMut<rules::StepCount>,
) {
    if args.step && keys.just_pressed(KeyCode::Space) {
        println!("Step");
        step_count.0 += 1;
    }
}
