use std::fmt::Display;

use crate::tile::Tile;
use crate::tile::letter::LETTERS;

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Bag {
    tiles: Vec<Tile>
}

impl Default for Bag {
    fn default() -> Self {
        let mut bag = Bag{
            tiles: Vec::<Tile>::new()
        };
    
        for (c, l) in LETTERS.into_iter(){
            for _ in 0..l.quantity {
                bag.tiles.push(Tile::new(*c));
            }
        }

        bag.tiles.shuffle(&mut thread_rng());

        return bag;
    }
}

impl Display for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bag[{}]:", self.tiles.len())?;
        for tile in &self.tiles {
            write!(f, "{}", tile)?;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests;