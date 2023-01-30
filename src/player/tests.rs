use super::tile::Tile;
use super::Hand;

#[test]
fn init_hand() {
    let hand = Hand::default();
    println!("hand: {:?}", hand);
}

#[test]
fn add_tile() {
    let tile = Tile::new('B', 1);
    let mut hand = Hand::default();
    println!("hand: {:?}", hand);

    assert_eq!(hand.tiles[0], None);

    hand.add_tile(tile, 0);

    assert_eq!(hand.tiles[0].unwrap(), tile);
}

#[test]
fn print_hand() {
    let hand = Hand::default();
    println!("hand: {}", hand);
}
