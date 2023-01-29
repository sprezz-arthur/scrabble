use std::fmt::{self, Display};

pub mod tile;

#[derive(Copy, Clone, Debug)]
enum Color {
    Beige,
    Pink,
    Red,
    Blue,
    LightBlue,
}

#[derive(Copy, Clone, Debug)]
pub enum Target {
    Word,
    Letter,
}

#[derive(Copy, Clone, Debug)]
pub struct CellProps {
    mult: Option<i32>,
    target: Option<Target>,
    color: Color,
}

impl CellProps {
    pub fn new(cell_type: CellType) -> CellProps {
        return match cell_type {
            Simple => CellProps {
                mult: None,
                target: None,
                color: Color::Beige,
            },
            Star => CellProps {
                mult: Some(2),
                target: Some(Target::Word),
                color: Color::Beige,
            },
            DoubleLetter => CellProps {
                mult: Some(2),
                target: Some(Target::Letter),
                color: Color::Beige,
            },
            TripleLetter => CellProps {
                mult: Some(3),
                target: Some(Target::Letter),
                color: Color::Beige,
            },
            DoubleWord => CellProps {
                mult: Some(2),
                target: Some(Target::Word),
                color: Color::Beige,
            },
            TripleWord => CellProps {
                mult: Some(3),
                target: Some(Target::Word),
                color: Color::Beige,
            },
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CellType {
    Star,
    Simple,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    tile: Option<tile::Tile>,
    props: CellProps,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Cell {
        let props = CellProps::new(cell_type);
        return Cell { tile: None, props };
    }
}

impl Default for Cell {
    fn default() -> Cell {
        return Cell::new(CellType::Simple);
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tile.is_none() {
            return write!(f, " ");
        }
        return write!(f, "{}", self.tile.unwrap());
    }
}

#[cfg(test)]
mod tests;
