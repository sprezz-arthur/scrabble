use std::fmt::{self, Display};

pub mod tile;

#[derive(Copy, Clone, Debug)]
enum Target {
    Word,
    Letter,
}

#[derive(Copy, Clone)]
pub struct Cell {
    tile: Option<tile::Tile>,
    mult: Option<i32>,
    target: Option<Target>,
}

impl Default for Cell {
    fn default() -> Cell {
        return Cell {
            tile: None,
            mult: None,
            target: None,
        };
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

#[cfg(test)]
mod tests;
