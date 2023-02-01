use std::fmt::{self, Display};

use letter::LETTERS;

pub mod letter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tile {
    pub letter: char,
    pub points: i32,
}

impl Tile {
    pub fn new(letter: char) -> Tile {
        return Tile {
            letter, points: LETTERS[&letter].points
         };
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.letter);
    }
}

impl Tile {
    pub fn repr(&self) -> String {
        return self.letter.to_string();
    }
}

#[cfg(test)]
mod tests;
