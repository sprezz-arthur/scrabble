use super::Tile;

#[test]
fn print_tile() {
    let tile = Tile::new('A', 1);
    println!("tile:\n{}", tile);
}
