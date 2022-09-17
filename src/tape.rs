use bevy::prelude::*;

#[derive(Clone)]
pub enum Alphabet {
    Blank,
    Zero,
    One,
}

#[derive(Component, Clone)]
pub struct Symbol(pub Alphabet);

#[derive(Component)]
pub struct TapeIndex(pub usize);
