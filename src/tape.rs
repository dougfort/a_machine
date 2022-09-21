use bevy::prelude::*;

#[derive(Component)]
pub struct Tape {
    pub symbols: Vec<String>,
    pub index: usize,
}

#[derive(Component)]
pub struct TapeIndex(pub usize);

impl Tape {
    pub fn new() -> Self {
        Tape {
            symbols: vec![" ".to_string()],
            index: 0,
        }
    }

    pub fn get(&self) -> &String {
        &self.symbols[self.index]
    }

    pub fn set(&mut self, symbol: &str) {
        self.symbols[self.index] = symbol.to_owned();
    }

    pub fn move_left(&mut self) {
        if self.index == 0 {
            self.symbols.insert(0, " ".to_string());
        } else {
            self.index -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.index == self.symbols.len() - 1 {
            self.symbols.push(" ".to_string());
        }
        self.index += 1;
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new()
    }
}
