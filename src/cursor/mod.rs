use super::BOARD_SIZE;

struct Position {
    x: usize,
    y: usize,
}

struct Cursor {
    pos: Position,
}

impl Cursor {
    fn default() -> Self {
        let pos = Position { x: 0, y: 0 };
        return Cursor { pos };
    }

    fn left(&mut self) {
        self.pos.x -= 1;
    }

    fn right(&mut self) {
        self.pos.x += 1;
    }

    fn up(&mut self) {
        self.pos.y -= 1;
    }

    fn down(&mut self) {
        self.pos.y += 1;
    }
}

/*
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
*/
