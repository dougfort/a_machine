use bevy::prelude::*;

pub enum Alphabet {
    Blank,
    Zero,
    One,
}

#[derive(Component)]
pub struct Symbol(pub Alphabet);

#[derive(Component)]
pub struct TapeIndex(pub usize);
