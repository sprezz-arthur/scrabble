use phf::phf_map;

#[derive(Clone)]
pub struct Letter {
    pub points: i32,
    pub quantity: i32
}

pub static LETTERS: phf::Map<char, Letter> = phf_map! {
    'A' => Letter {points: 1, quantity: 9},
    'B' => Letter {points: 3, quantity: 2},
    'C' => Letter {points: 3, quantity: 2},
    'D' => Letter {points: 2, quantity: 4},
    'E' => Letter {points: 1, quantity: 12},
    'F' => Letter {points: 4, quantity: 2},
    'G' => Letter {points: 2, quantity: 3},
    'H' => Letter {points: 4, quantity: 2},
    'I' => Letter {points: 1, quantity: 9},
    'J' => Letter {points: 8, quantity: 1},
    'K' => Letter {points: 5, quantity: 1},
    'L' => Letter {points: 1, quantity: 4},
    'M' => Letter {points: 3, quantity: 2},
    'N' => Letter {points: 1, quantity: 6},
    'O' => Letter {points: 1, quantity: 8},
    'P' => Letter {points: 3, quantity: 2},
    'Q' => Letter {points: 10, quantity: 1},
    'R' => Letter {points: 1, quantity: 6},
    'S' => Letter {points: 1, quantity: 4},
    'T' => Letter {points: 1, quantity: 6},
    'U' => Letter {points: 1, quantity: 4},
    'V' => Letter {points: 4, quantity: 2},
    'W' => Letter {points: 4, quantity: 2},
    'X' => Letter {points: 8, quantity: 1},
    'Y' => Letter {points: 4, quantity: 2},
    'Z' => Letter {points: 10, quantity: 1},
    ' ' => Letter {points: 0, quantity: 2},
};

#[cfg(test)]
mod tests;