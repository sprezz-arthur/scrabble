use super::Bag;
use super::Tile;

#[test]
fn print_bag() {
    let bag = Bag::default();
    println!("bag:\n{}", bag);
}

#[test]
fn test_empty_bag(){
    let mut bag = Bag::empty();
    assert_eq!(bag.tiles.len(), 0);
}


#[test]
fn test_return_tiles(){
    let mut bag = Bag::empty();
    let mut tiles = [
        Tile::new('A'),
        Tile::new('B'),
        Tile::new('C'),
    ].to_vec();

    bag.return_tiles(&mut tiles);

    assert_eq!(bag.tiles.len(), 3);
    assert_eq!(tiles.len(), 0);
}

