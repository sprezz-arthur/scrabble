use super::*;
use rstest::{rstest, fixture};

#[fixture]
fn dict() -> Vec<String> {
    return lines_from_file("/home/arthur/projects/scrabble-rust/src/dictionary/dictionary.txt");
}


#[rstest]
fn check_pair(
    #[values(
        ("avocado", true),
        ("bubblegum", true),
        ("chocolate", true),
        ("data", true))]
    pair: (&str, bool),
) {
    let (word, expected) = pair;
    assert_eq!(in_dict(word), expected);
}

#[rstest]
fn word_in_dict(#[values("avocado", "bubblegum", "chocolate", "data")] word: &str) {
    assert_eq!(in_dict(word), true);
}

#[rstest]
fn word_not_in_dict(#[values("abacate", "babalu", "chicote", "dadinho")] word: &str) {
    assert_eq!(in_dict(word), false);
}
