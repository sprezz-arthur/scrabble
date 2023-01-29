use itertools::Itertools;

pub mod cell;

const BOARD_SIZE: usize = 15;

pub struct Board([[cell::Cell; BOARD_SIZE]; BOARD_SIZE]);

use std::fmt::{self, Display};

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            writeln!(f, "{}", row.iter().join("|"))?;
        }
        return Ok(());
    }
}

impl Default for Board {
    fn default() -> Board {
        let board = Board([[cell::Cell::default(); BOARD_SIZE]; BOARD_SIZE]);
        return board;
    }
}

#[cfg(test)]
mod tests;
