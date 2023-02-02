mod bag;
mod board;
mod dictionary;
mod game;
mod player;
mod tile;

fn main() {
    let game = game::init_game(2);

    println!("{}", game);
}
