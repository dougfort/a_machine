use bevy::prelude::*;

pub struct SpriteArray([Handle<Image>; 3]);

#[derive(Component)]
pub struct SpriteSymbol(pub String);

impl FromWorld for SpriteArray {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.get_resource::<AssetServer>().unwrap();
        let zero_sprite: Handle<Image> = asset_server.load("zero.png");
        let one_sprite: Handle<Image> = asset_server.load("one.png");
        let blank_sprite: Handle<Image> = asset_server.load("blank.png");

        SpriteArray([zero_sprite, one_sprite, blank_sprite])
    }
}

impl SpriteArray {
    pub fn get(&self, symbol: &str) -> Handle<Image> {
        match symbol {
            "0" => self.0[0].clone(),
            "1" => self.0[1].clone(),
            _ => self.0[2].clone(),
        }
    }
}
