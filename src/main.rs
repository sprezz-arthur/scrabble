mod bag;
mod board;
mod dictionary;
mod game;
mod player;
mod tile;

use std::io::{self};

use std::io::{stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut game = game::init_game(2);
    let (mut x, mut y) = (0, 0);
    let (mut prev_x, mut prev_y) = (0, 0);

    let stdin = stdin();
    let _out = stdout().into_raw_mode();

    print!("\x1B[2J\x1B[1;1H{}{}", game, termion::cursor::Hide);
    game.board.pointer(x, y, prev_x, prev_y);
    for c in stdin.keys() {
        print!("\x1B[2J\x1B[1;1H{}{}", game, termion::cursor::Hide);
        let key = c.unwrap();
        match key {
            Key::Up => {
                y -= 1;
            }
            Key::Down => {
                y += 1;
            }
            Key::Left => {
                x -= 1;
            }
            Key::Right => {
                x += 1;
            }
            Key::Esc => break,
            Key::Char(letter) => {
                game.board.place_letter(letter, x, y);
            }
            _ => {}
        }
        game.board.pointer(x, y, prev_x, prev_y);
        (prev_x, prev_y) = (x, y);
    }
}
