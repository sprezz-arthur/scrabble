use itertools::Itertools;

use std::fmt::{self, Debug, Display};

use crate::tile::Tile;

pub mod cell;

const BOARD_SIZE: usize = 15;

#[derive(Debug)]
pub struct Board([[cell::Cell; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn pointer(&self, x: usize, y: usize, prev_x: usize, prev_y: usize) {
        let pointer = " ✎ ";
        let prev_cell = self.0[prev_x][prev_y];

        println!(
            "{}{}",
            termion::cursor::Goto(3 * (prev_x + 1) as u16, (prev_y + 3) as u16),
            prev_cell
        );
        println!(
            "{}{}{}",
            termion::cursor::Goto(3 * (x + 1) as u16, (y + 3) as u16),
            pointer,
            termion::cursor::Hide
        );
    }

    pub fn place_letter(&mut self, letter: char, x: usize, y: usize) {
        self.0[x][y].tile = Some(Tile::new(letter));
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\r  ")?;
        writeln!(f, " 0  1  2  3  4  5  6  7  8  9  A  B  C  D  E \r")?;
        write!(f, "{}\r", "\x1b[0m")?;
        for (i, row) in self.0.iter().enumerate() {
            if i < 10 {
                write!(f, "{} ", i)?;
            } else {
                write!(f, "{} ", (i as u8 + 55) as char)?;
            }
            writeln!(f, "{}", row.iter().join("") + "\r")?;
        }
        write!(
            f,
            "  \x1b[1;53;30m{}\x1b[0m",
            " ".repeat(3 * BOARD_SIZE) + "\r"
        )?;

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
