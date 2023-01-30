use super::Board;

#[test]
fn print_board() {
    let board = Board::default();
    println!("board:\n{}", board);
}
