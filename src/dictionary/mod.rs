use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


pub fn in_dict(word: &str) -> bool {
    let dict = lines_from_file("./src/dictionary/dictionary.txt");
    return dict.contains(&word.to_string().to_uppercase());
}

#[cfg(test)]
mod tests;
