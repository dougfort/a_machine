use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum Alphabet {
    Blank,
    Zero,
    One,
}

#[derive(Component, Clone, Debug)]
pub struct Symbol(pub Alphabet);

#[derive(Component)]
pub struct Tape{
    pub symbols: Vec<Alphabet>,
    pub index: usize,
}

#[derive(Component)]
pub struct TapeIndex(pub usize);

impl Tape {
    pub fn new() -> Self {
        Tape {
            symbols: vec![Alphabet::Blank],
            index: 0,
        }
    }

    pub fn get(&self) -> &Alphabet {
        &self.symbols[self.index]
    }

    pub fn set(&mut self, symbol: Alphabet) {
        self.symbols[self.index] = symbol;
    }

    pub fn move_left(&mut self) {
        if self.index == 0 {
            self.symbols.insert(0, Alphabet::Blank);
        } else {
            self.index -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.index == self.symbols.len() - 1 {
            self.symbols.push(Alphabet::Blank);
        }
        self.index += 1;
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new()
    }
}
