use bevy::prelude::*;

use crate::cli;
use crate::step;

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    args: Res<cli::Args>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    if args.step && keys.just_pressed(KeyCode::Space) {
        step(commands, asset_server);
    }
}
