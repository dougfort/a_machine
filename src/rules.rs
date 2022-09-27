/// transition rules
use bevy::{prelude::*, utils::HashMap};
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

use crate::sprites;
use crate::state;
use crate::tape;

use crate::center_sprite_index;
#[derive(Deserialize, Debug)]
pub struct Rule {
    from_state: String,
    from_symbol: String,
    to_state: String,
    to_symbol: String,
    direction: String,
}

pub struct RuleSet(HashMap<(String, String), Rule>);
#[derive(Debug)]
pub struct StepCount(pub usize);

#[derive(Debug)]
pub struct EntityArray(pub Vec<(Entity, f32)>);

impl FromWorld for RuleSet {
    fn from_world(world: &mut World) -> Self {
        let args = world.get_resource::<crate::cli::Args>().unwrap();

        let path = &args.rules_file;
        let file = File::open(path).expect("file not found");
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as a Vector or Rule`.
        let rules: Vec<Rule> = serde_json::from_reader(reader).expect("file is not valid JSON");

        let rule_set = HashMap::from_iter(
            rules
                .into_iter()
                .map(|rule| ((rule.from_state.clone(), rule.from_symbol.clone()), rule)),
        );

        RuleSet(rule_set)
    }
}

pub fn step(
    step_count: Res<StepCount>,
    mut commands: Commands,
    rule_set: Res<RuleSet>,
    mut state: ResMut<state::State>,
    mut tape: ResMut<tape::Tape>,
    sprite_array: Res<sprites::SpriteArray>,
    entity_array: Res<EntityArray>,
) {
    if !step_count.is_changed() {
        return;
    }
    let from_state = state.0.clone();
    let from_symbol = tape.get().clone();
    let rule = rule_set.0.get(&(from_state, from_symbol));
    if let Some(rule) = rule {
        println!(
            "Step: {}; State: {}; Rule: {:?}",
            step_count.0, state.0, rule
        );

        tape.set(&rule.to_symbol);
        state.0 = rule.to_state.clone();
        if rule.direction == "L" {
            tape.move_left();
        } else if rule.direction == "R" {
            tape.move_right();
        }

        // clear the sprites
        entity_array.0.iter().for_each(|(entity, _)| {
            commands.entity(*entity).remove_bundle::<SpriteBundle>();
        });

        // set the current tape head over the center sprite
        let tape_head_index = tape.index;
        let center_sprite_index = center_sprite_index();

        println!("tape_head_index: {}; center_sprite_index: {}; tape: {:?}", tape_head_index, center_sprite_index, tape.symbols);

        let symbol = tape.get_at(tape_head_index);
        let (entity, x) = entity_array.0[center_sprite_index];
        commands.entity(entity).insert_bundle(SpriteBundle {
            texture: sprite_array.get(symbol),
            visibility: Visibility { is_visible: true },
            transform: Transform::from_translation(Vec3::new(x, 0.0, 0.0)),
            ..Default::default()
        });

        for i in 0..center_sprite_index {
            let j = center_sprite_index - i - 1;
            println!("i: {}; j: {}", i, j);
            let k = tape_head_index - i - 1;
            let symbol = tape.get_at(k);
            let (entity, x) = entity_array.0[j];
            commands.entity(entity).insert_bundle(SpriteBundle {
                texture: sprite_array.get(symbol),
                visibility: Visibility { is_visible: true },
                transform: Transform::from_translation(Vec3::new(x, 0.0, 0.0)),
                ..Default::default()
            });
            if k == 0 {
                break;
            }
        }
    } else {
        println!("No rule for State: {:?}; Symbol: {:?}", state, tape.get());
    }
}
