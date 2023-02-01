use super::{LETTERS};

#[test]
fn total_letters_count(){
    let total_count: i32 = LETTERS.into_iter().map(|(_c, l)| l.quantity).sum();
    assert_eq!(total_count, 100);
}

#[test]
fn total_letters_points(){
    let total_points: i32 = LETTERS.into_iter().map(|(_c, l)| l.quantity*l.points).sum();
    assert_eq!(total_points, 187);
}