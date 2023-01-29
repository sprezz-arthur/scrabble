use super::Board;

#[test]
fn board_is_printable() {
    let board = Board::default();
    println!("board:\n{}", board);
}
