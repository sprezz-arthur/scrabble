use std::fmt::{self, Display};

use crate::tile;

#[derive(Copy, Clone, Debug)]
enum Color {
    Beige,
    Salmon,
    Red,
    Blue,
    LightBlue,
}

impl Color {
    pub fn hex_code(&self) -> &str {
        return match &self {
            Color::Beige => "\x1b[107m",
            Color::LightBlue => "\x1b[106m",
            Color::Blue => "\x1b[44m",
            Color::Salmon => "\x1b[43m",
            Color::Red => "\x1b[41m",
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Target {
    Word,
    Letter,
}

#[derive(Copy, Clone, Debug)]
pub struct CellProps {
    symbol: char,
    mult: Option<i32>,
    target: Option<Target>,
    color: Color,
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

impl CellProps {
    pub fn new(cell_type: CellType) -> CellProps {
        return match cell_type {
            CellType::Simple => CellProps {
                symbol: ' ',
                mult: None,
                target: None,
                color: Color::Beige,
            },
            CellType::Star => CellProps {
                symbol: '*',
                mult: Some(2),
                target: Some(Target::Word),
                color: Color::Salmon,
            },
            CellType::DoubleLetter => CellProps {
                symbol: ' ',
                mult: Some(2),
                target: Some(Target::Letter),
                color: Color::LightBlue,
            },
            CellType::TripleLetter => CellProps {
                symbol: ' ',
                mult: Some(3),
                target: Some(Target::Letter),
                color: Color::Blue,
            },
            CellType::DoubleWord => CellProps {
                symbol: ' ',
                mult: Some(2),
                target: Some(Target::Word),
                color: Color::Salmon,
            },
            CellType::TripleWord => CellProps {
                symbol: ' ',
                mult: Some(3),
                target: Some(Target::Word),
                color: Color::Red,
            },
        };
    }
}

impl CellProps {
    pub fn hex_code(&self) -> &str {
        return self.color.hex_code();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub tile: Option<tile::Tile>,
    props: CellProps,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Cell {
        let props = CellProps::new(cell_type);
        return Cell {
            tile: None,
            props: props,
        };
    }
}

impl Default for Cell {
    fn default() -> Cell {
        return Cell::new(CellType::Simple);
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = if self.tile.is_none() {
            "▏·▕".replace("·", &self.props.symbol.to_string())
        } else {
            "▏·▕".replace("·", &self.tile.unwrap().repr())
        };
        write!(f, "{}", "\x1b[1;53;30m")?;
        write!(f, "{}", self.props.color.hex_code())?;
        write!(f, "{}", repr)?;
        write!(f, "{}", "\x1b[0m")?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests;
