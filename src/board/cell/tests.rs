use super::Cell;

#[test]
fn cell_is_printable() {
    let cell = Cell::default();
    println!("cell:\n{}", cell);
}
