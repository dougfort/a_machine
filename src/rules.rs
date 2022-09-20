/// transition rules
use bevy::{prelude::*, utils::HashMap};

use crate::state;
use crate::tape;

#[derive(Debug)]
pub struct Rule {
    from_state: state::State,
    from_symbol: tape::Alphabet,
    to_state: state::State,
    to_symbol: tape::Alphabet,
    direction: String,
}

pub struct RuleSet(HashMap<(state::State, tape::Alphabet), Rule>);

impl FromWorld for RuleSet {
    fn from_world(_world: &mut World) -> Self {
        // Turing's very first example
        // https://en.wikipedia.org/wiki/Turing_machine_examples
        let rules = vec![
            Rule {
                from_state: state::State("b".to_string()),
                from_symbol: tape::Alphabet::Blank,
                to_state: state::State("c".to_string()),
                to_symbol: tape::Alphabet::Zero,
                direction: "R".to_string(),
            },
            Rule {
                from_state: state::State("c".to_string()),
                from_symbol: tape::Alphabet::Blank,
                to_state: state::State("e".to_string()),
                to_symbol: tape::Alphabet::Blank,
                direction: "R".to_string(),
            },
            Rule {
                from_state: state::State("e".to_string()),
                from_symbol: tape::Alphabet::Blank,
                to_state: state::State("f".to_string()),
                to_symbol: tape::Alphabet::One,
                direction: "R".to_string(),
            },
            Rule {
                from_state: state::State("f".to_string()),
                from_symbol: tape::Alphabet::Blank,
                to_state: state::State("b".to_string()),
                to_symbol: tape::Alphabet::Blank,
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
    let from_state = state.clone();
    let from_symbol = tape.get().clone();
    let rule = rule_set.0.get(&(from_state, from_symbol));
    if let Some(rule) = rule {
        println!("State: {:?}; Rule: {:?}", state, rule);
        tape.set(rule.to_symbol.clone());
        state.0 = rule.to_state.0.to_string();
        if rule.direction == "L" {
            tape.move_left();
        } else if rule.direction == "R" {
            tape.move_right();
        }
    }
}
