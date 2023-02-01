use super::{LETTERS};

#[test]
fn total_letters_count(){
    let total_count: i32 = LETTERS.into_iter().map(|(_key, letter)| letter.quantity).sum();
    assert_eq!(total_count, 100);
}

#[test]
fn total_letters_points(){
    let total_points: i32 = LETTERS.into_iter().map(|(_key, letter)| letter.quantity*letter.points).sum();
    assert_eq!(total_points, 187);
}