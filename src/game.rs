use std::fmt::Display;

use crate::bag;
use crate::board;
use crate::dictionary;
use crate::game;
use crate::player;
use crate::tile;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<player::Player>,
    pub board: board::Board,
    pub bag: bag::Bag,
}

pub fn init_game(num_players: i32) -> Game {
    let mut players = Vec::new();
    for i in 0..num_players {
        players.push(player::Player::new(i));
    }

    let board = board::Board::default();
    let bag = bag::Bag::default();

    return Game {
        players,
        board,
        bag,
    };
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Board:\n{}\n", self.board)?;
        writeln!(f, "Bag:\n{}\n", self.bag)?;

        for (i, player) in self.players.iter().enumerate() {
            writeln!(f, "Player {}:\n{}", i, player)?;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    fn test_init_game(#[values(1, 2, 3, 4)] num_players: i32) {
        init_game(num_players);
    }
}
