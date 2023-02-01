use std::fmt::{self, Display};

mod letter;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tile {
    pub letter: char,
    pub value: i32,
}

impl Tile {
    pub fn new(letter: char, value: i32) -> Tile {
        return Tile { letter, value };
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
