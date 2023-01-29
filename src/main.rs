mod board;

fn main() {
    let board = board::Board::default();
    println!("This is my board: \n{}", board);
}
