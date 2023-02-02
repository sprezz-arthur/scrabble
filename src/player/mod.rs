use std::fmt::Display;

mod hand;

#[derive(Debug)]
pub struct Player {
    id: i32,
    hand: hand::Hand,
    score: i32,
}

impl Player {
    pub fn new(id: i32) -> Self {
        return Player {
            id,
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
