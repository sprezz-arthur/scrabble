use super::Tile;

#[test]
fn tile_is_printable() {
    let tile = Tile::new('A', 1);
    println!("tile:\n{}", tile);
}
