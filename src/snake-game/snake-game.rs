extern crate ncurses;
extern crate rand;
use ncurses::*;
use std::thread::sleep;
use std::time::Duration;
mod utils;
use utils::{Direction, SnakeGame};

pub fn snake_game() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(100);

    let mut game = SnakeGame::new();

    loop {
        game.draw();
        let input = getch();
        if input == 'q' as i32 {
            break;
        }

        match input {
            KEY_UP => game.change_direction(Direction::Up),
            KEY_DOWN => game.change_direction(Direction::Down),
            KEY_LEFT => game.change_direction(Direction::Left),
            KEY_RIGHT => game.change_direction(Direction::Right),
            _ => (),
        }

        if !game.update() {
            break;
        }

        sleep(Duration::from_millis(150));
    }

    endwin();
}
