mod board;

fn main() {
    let cell = board::cell::Cell::default();
    println!("This is how my cell looks: {}", cell);
    println!("This is my cell's content: {:?}", cell);
}
