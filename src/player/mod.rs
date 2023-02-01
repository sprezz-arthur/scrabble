use std::fmt::{self, Display};

use itertools::Itertools;

use crate::tile;

const HAND_SIZE: usize = 7;

#[derive(Debug)]
pub struct Hand {
    tiles: [Option<tile::Tile>; HAND_SIZE],
}

impl Default for Hand {
    fn default() -> Hand {
        return Hand {
            tiles: [None; HAND_SIZE],
        };
    }
}

impl Hand {
    pub fn add_tile(&mut self, tile: tile::Tile, pos: usize) {
        self.tiles[pos] = Some(tile);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tiles
                .iter()
                .map(|&x| if x.is_none() { '_' } else { x.unwrap().letter })
                .join(" ")
        )?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests;
