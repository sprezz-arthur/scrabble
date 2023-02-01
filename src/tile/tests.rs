use super::{Tile};

use super::letter::{LETTER_DISTRIBUTION, LETTER_VALUES};

#[test]
fn print_tile() {
    let tile = Tile::new('A', 1);
    println!("tile:\n{}", tile);
}

#[test]
fn total_letters_count(){
    let total_count: i32 = LETTER_DISTRIBUTION.map(|l| l.1).iter().sum();
    assert_eq!(total_count, 100);
}

#[test]
fn total_letters_value(){
    let mut total_value = 0;
    for (count, value) in LETTER_DISTRIBUTION.iter().zip(LETTER_VALUES.iter()){
        total_value += count.1*value.1;
    }
    assert_eq!(total_value, 187);
}