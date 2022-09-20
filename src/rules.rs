/// transition rules
use bevy::{prelude::*, utils::HashMap};

use crate::state;
use crate::tape;

#[derive(Debug)]
pub struct Rule {
    from_state: String,
    from_symbol: String,
    to_state: String,
    to_symbol: String,
    direction: String,
}

pub struct RuleSet(HashMap<(String, String), Rule>);

impl FromWorld for RuleSet {
    fn from_world(_world: &mut World) -> Self {
        // Turing's very first example
        // https://en.wikipedia.org/wiki/Turing_machine_examples
        let rules = vec![
            Rule {
                from_state: "b".to_string(),
                from_symbol: " ".to_string(),
                to_state: "c".to_string(),
                to_symbol: "0".to_string(),
                direction: "R".to_string(),
            },
            Rule {
                from_state: "c".to_string(),
                from_symbol: " ".to_string(),
                to_state: "e".to_string(),
                to_symbol: " ".to_string(),
                direction: "R".to_string(),
            },
            Rule {
                from_state: "e".to_string(),
                from_symbol: " ".to_string(),
                to_state: "f".to_string(),
                to_symbol: "1".to_string(),
                direction: "R".to_string(),
            },
            Rule {
                from_state: "f".to_string(),
                from_symbol: " ".to_string(),
                to_state: "b".to_string(),
                to_symbol: " ".to_string(),
                direction: "R".to_string(),
            },
        ];
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
