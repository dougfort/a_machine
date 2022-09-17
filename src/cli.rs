use bevy::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "a_machine", about = "Automatic Machine")]
pub struct Args {
    /// start in step mode
    /// (press space to advance one step)
    #[structopt(short, long)]
    pub step: bool,
}

impl FromWorld for Args {
    fn from_world(_world: &mut World) -> Self {
        Args::from_args()
    }
}
