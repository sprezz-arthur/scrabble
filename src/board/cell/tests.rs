use super::{Cell, CellType, Target};

#[test]
fn cell_is_printable() {
    let cell = Cell::default();
    println!("cell:\n{}", cell);
}

#[test]
fn initialize_simple_cell() {
    let cell = Cell::new(CellType::Simple);
    assert!(cell.props.mult == None);
    assert!(cell.props.target == None);
    println!("cell:\n{:?}", cell);
}

#[test]
fn initialize_star_cell() {
    let cell = Cell::new(CellType::Star);
    assert!(cell.props.mult == Some(2));
    assert!(cell.props.target == Some(Target::Word));
    println!("cell:\n{:?}", cell);
}
