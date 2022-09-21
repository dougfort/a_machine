/// transition rules
use bevy::{prelude::*, utils::HashMap};
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

use crate::state;
use crate::tape;

#[derive(Deserialize, Debug)]
pub struct Rule {
    from_state: String,
    from_symbol: String,
    to_state: String,
    to_symbol: String,
    direction: String,
}

pub struct RuleSet(HashMap<(String, String), Rule>);

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

pub fn step(rule_set: Res<RuleSet>, mut state: ResMut<state::State>, mut tape: ResMut<tape::Tape>) {
    let from_state = state.0.clone();
    let from_symbol = tape.get().clone();
    let rule = rule_set.0.get(&(from_state, from_symbol));
    if let Some(rule) = rule {
        println!("State: {:?}; Rule: {:?}", state, rule);
        tape.set(&rule.to_symbol);
        state.0 = rule.to_state.clone();
        if rule.direction == "L" {
            tape.move_left();
        } else if rule.direction == "R" {
            tape.move_right();
        }
    }
}
