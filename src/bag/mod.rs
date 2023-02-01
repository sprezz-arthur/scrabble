use std::fmt::Display;

use crate::tile::Tile;

struct Bag {
    tiles: Vec<Tile>
}

impl Default for Bag {
    fn default() -> Self {
        return Bag{
            tiles: Vec::<Tile>::new()
        }
    }
}

impl Display for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tile in &self.tiles {
            write!(f, "{}", tile)?;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests;