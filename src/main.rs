mod bag;
mod board;
mod dictionary;
mod game;
mod player;
mod tile;

use std::io;

fn main() {
    let game = game::init_game(2);
    let mut guess = String::new();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", game);
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");
    }
}
