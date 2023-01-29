mod cell;

const BOARD_SIZE: usize = 15;

pub struct Board([[cell::Cell; BOARD_SIZE]; BOARD_SIZE]);

use std::fmt::{self, Display};

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            for cell in row.iter() {
                write!(f, "{} ", cell)?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}

impl Default for Board {
    fn default() -> Board {
        return Board([[cell::Cell::default(); BOARD_SIZE]; BOARD_SIZE]);
    }
}

#[cfg(test)]
mod tests;
