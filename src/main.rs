mod board;

fn main() {
    let mut cell = board::cell::Cell::new(board::cell::CellType::Star);
    println!("This is how my cell looks: {}", cell);
    println!("This is my cell's content: {:?}", cell);

    println!("\nAnd after I place an `A` tile, it goes...\n");

    let tile = board::cell::tile::Tile::new('A', 1);

    cell.tile = Some(tile);

    println!("This is how my cell looks: {}", cell);
    println!("This is my cell's content: {:?}", cell);


    let board = board::Board::default();
    println!("This is my board:\n{}", board);
}
