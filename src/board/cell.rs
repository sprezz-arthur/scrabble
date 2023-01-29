use std::fmt::{self, Display};

pub mod tile;

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
            Beige => "\x1b[44m",
            LightBlue => "\x1b[46m",
            Blue => "\x1b[44m",
            Salmon => "\x1b[45m",
            Red => "\x1b[41m",
        };
    }
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
                color: Color::Salmon,
            },
            DoubleLetter => CellProps {
                mult: Some(2),
                target: Some(Target::Letter),
                color: Color::LightBlue,
            },
            TripleLetter => CellProps {
                mult: Some(3),
                target: Some(Target::Letter),
                color: Color::Blue,
            },
            DoubleWord => CellProps {
                mult: Some(2),
                target: Some(Target::Word),
                color: Color::Salmon,
            },
            TripleWord => CellProps {
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
    pub tile: Option<tile::Tile>,
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
        let repr = if self.tile.is_none() {
            " ".to_string()
        } else {
            self.tile.unwrap().repr()
        };

        write!(f, "{}", self.props.color.hex_code())?;
        write!(f, "{}", repr)?;
        write!(f, "{}", "\x1b[0m")?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests;
