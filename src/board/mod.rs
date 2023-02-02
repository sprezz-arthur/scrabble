use itertools::Itertools;

pub mod cell;

const BOARD_SIZE: usize = 15;

#[derive(Debug)]
pub struct Board([[cell::Cell; BOARD_SIZE]; BOARD_SIZE]);

use std::fmt::{self, Display};

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  ")?;
        write!(f, "{}", "\x1b[4m")?;
        writeln!(f, " 0  1  2  3  4  5  6  7  8  9  A  B  C  D  E ")?;
        write!(f, "{}", "\x1b[0m")?;
        for (i, row) in self.0.iter().enumerate() {
            if i < 10 {
                write!(f, "{} ", i)?;
            } else {
                write!(f, "{} ", (i as u8 + 55) as char)?;
            }
            writeln!(f, "{}", row.iter().join(""))?;
        }
        return Ok(());
    }
}

impl Default for Board {
    fn default() -> Board {
        let mut board = Board([[cell::Cell::default(); BOARD_SIZE]; BOARD_SIZE]);

        board.0[0][0] = cell::Cell::new(cell::CellType::TripleWord);
        board.0[0][3] = cell::Cell::new(cell::CellType::DoubleLetter);
        board.0[0][7] = cell::Cell::new(cell::CellType::TripleWord);

        board.0[1][1] = cell::Cell::new(cell::CellType::DoubleWord);
        board.0[1][5] = cell::Cell::new(cell::CellType::TripleLetter);

        board.0[2][2] = cell::Cell::new(cell::CellType::DoubleWord);
        board.0[2][6] = cell::Cell::new(cell::CellType::DoubleLetter);

        board.0[3][3] = cell::Cell::new(cell::CellType::DoubleWord);
        board.0[3][5] = cell::Cell::new(cell::CellType::DoubleLetter);

        board.0[4][4] = cell::Cell::new(cell::CellType::DoubleWord);

        board.0[5][5] = cell::Cell::new(cell::CellType::TripleLetter);

        board.0[6][6] = cell::Cell::new(cell::CellType::DoubleLetter);

        board.0[7][7] = cell::Cell::new(cell::CellType::Star);

        for (i, row) in board.0.into_iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if j > i {
                    board.0[j][i] = cell.clone();
                }
            }
        }

        for (i, row) in board.0.into_iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if i < BOARD_SIZE / 2 {
                    board.0[BOARD_SIZE - i - 1][j] = cell.clone();
                }
            }
        }

        for (i, row) in board.0.into_iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if j < BOARD_SIZE / 2 {
                    board.0[i][BOARD_SIZE - j - 1] = cell.clone();
                }
            }
        }
        return board;
    }
}

#[cfg(test)]
mod tests;
