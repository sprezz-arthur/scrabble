use std::fmt::Display;

use rand::seq::SliceRandom;

mod hand;

#[derive(Debug)]
pub struct Player {
    id: i32,
    pub icon: char,
    pub hand: hand::Hand,
    pub score: i32,
}

impl Player {
    pub fn new(id: i32, icon: char) -> Self {
        return Player {
            id,
            icon,
            hand: hand::Hand::default(),
            score: 0,
        };
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self);
        return Ok(());
    }
}

#[cfg(test)]
mod tests;
