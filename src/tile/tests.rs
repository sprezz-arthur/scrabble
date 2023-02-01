use super::{Tile};


#[test]
fn print_tile() {
    let tile = Tile::new('A');
    println!("tile:\n{}", tile);
}