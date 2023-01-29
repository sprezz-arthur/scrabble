use super::{Cell, CellType};

#[test]
fn cell_is_printable() {
    let cell = Cell::default();
    println!("cell:\n{}", cell);
}

#[test]
fn initialize_simple_cell() {
    let cell = Cell::new(CellType::Simple);
    println!("cell:\n{:?}", cell);
}

#[test]
fn initialize_star_cell() {
    let cell = Cell::new(CellType::Star);
    println!("cell:\n{:?}", cell);
}
