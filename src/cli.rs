use bevy::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "a_machine", about = "Automatic Machine")]
pub struct Args {
    /// start in step mode
    /// (press space to advance one step)
    #[structopt(short, long)]
    pub step: bool,

    /// rules file JSON
    /// (default: rules.json)
    #[structopt(short, long, default_value = "assets/rules.json")]
    pub rules_file: String,

    /// initial state
    /// (default: b)
    #[structopt(short, long, default_value = "b")]
    pub initial_state: String,
}

impl FromWorld for Args {
    fn from_world(_world: &mut World) -> Self {
        Args::from_args()
    }
}
