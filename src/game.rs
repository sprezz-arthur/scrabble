use std::fmt::{Debug, Display};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::bag;
use crate::board;
use crate::player;

pub struct Game {
    pub players: Vec<player::Player>,
    pub board: board::Board,
    pub bag: bag::Bag,
}

pub fn init_game(num_players: i32) -> Game {
    let mut players = Vec::new();

    let icons = ['💩', '🤡', '👹', '👺', '👻', '👽', '👾', '🤖'];
    let mut icons = icons.to_vec();
    icons.shuffle(&mut thread_rng());

    for i in 0..num_players {
        let icon = icons[i as usize];
        players.push(player::Player::new(i, icon));
    }

    let board = board::Board::default();
    let bag = bag::Bag::default();

    return Game {
        players,
        board,
        bag,
    };
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Board:\n{}\n", self.board)?;
        writeln!(f, "Bag:\n{}\n", self.bag)?;

        for (i, player) in self.players.iter().enumerate() {
            writeln!(f, "Player {}:\n{}", i, player)?;
        }
        return Ok(());
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[2J\x1B[1;1H")?;
        writeln!(f, "Board:\n\r{}\n\r", self.board)?;
        writeln!(f, "💰 Tiles reamining: {}\n\r", self.bag)?;

        for (i, player) in self.players.iter().enumerate() {
            writeln!(
                f,
                "{} Player {}:\n\r\t{}\n\r\tScore:{}\n\r",
                player.icon, i, player.hand, player.score
            )?;
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
