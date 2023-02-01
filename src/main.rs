mod bag;
mod board;
mod dictionary;
mod player;
mod tile;

fn main() {
    let mut cell = board::cell::Cell::new(board::cell::CellType::Star);
    println!("This is how my cell looks: {}", cell);
    println!("This is my cell's content: {:?}", cell);

    println!("\nAnd after I place an `A` tile, it goes...\n");

    let tile = tile::Tile::new('A');

    cell.tile = Some(tile);

    println!("This is how my cell looks: {}", cell);
    println!("This is my cell's content: {:?}", cell);

    let board = board::Board::default();
    println!("This is my board:\n\n{}", board);

    let mut hand = player::Hand::default();

    println!("{}", hand);
    println!("{:?}\n", hand);

    hand.add_tile(tile, 0);

    println!("{}", hand);
    println!("{:?}\n", hand);

    let mut bag = bag::Bag::empty();


    println!("{}", bag);
    println!("{:?}\n", bag);

    let tile = bag.draw_tile();
    
    if tile.is_none() {
        println!("Bag is empty!");
    }
    else {
        println!("drew: {:?}", tile); 
    }

}
