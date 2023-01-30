use super::*;
use rstest::rstest;

#[rstest]
fn check_pair(
    #[values(
        ("avocado", true),
        ("bubblegum", true),
        ("chocolat", true),
        ("data", true))]
    pair: (&str, bool),
) {
    let (word, expected) = pair;
    assert_eq!(in_dict(word), expected);
}

#[rstest]
fn word_in_dict(#[values("avocado", "bubblegum", "chocolat", "data")] word: &str) {
    assert_eq!(in_dict(word), true);
}

#[rstest]
fn word_not_in_dict(#[values("abacate", "babalu", "chicote", "dadinho")] word: &str) {
    assert_eq!(in_dict(word), true);
}
