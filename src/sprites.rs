use bevy::prelude::*;

use crate::tape;

pub struct Sprites([Handle<Image>; 3]);

impl FromWorld for Sprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.get_resource::<AssetServer>().unwrap();
        let zero_sprite: Handle<Image> = asset_server.load("zero.png");
        let one_sprite: Handle<Image> = asset_server.load("one.png");
        let blank_sprite: Handle<Image> = asset_server.load("blank.png");

        Sprites([zero_sprite, one_sprite, blank_sprite])
    }
}

impl Sprites {
    pub fn get(&self, symbol: &tape::Alphabet) -> Handle<Image> {
        match symbol {
            tape::Alphabet::Zero => self.0[0].clone(),
            tape::Alphabet::One => self.0[1].clone(),
            _ => self.0[2].clone(),
        }
    }
}
