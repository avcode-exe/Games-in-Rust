extern crate ncurses;
extern crate rand;
use ncurses::*;
#[path = "maze/maze.rs"]
mod maze;
#[path = "minesweeper/minewseeper.rs"]
mod minesweeper;
#[path = "snake-game/snake-game.rs"]
mod snake_game;

fn main() {
    let _games = vec![
        ("Maze", maze::maze as fn()),
        ("Minesweeper", minesweeper::minesweeper as fn()),
        ("Snake", snake_game::snake_game as fn()),
        ("Quit", || {
            endwin();
            std::process::exit(0);
        }),
    ];

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(100);

    let mut selected = 0;
    loop {
        clear();
        let _ = mvprintw(0, 0, "Select a game or quit:");
        for (i, &(name, _)) in _games.iter().enumerate() {
            let ch = if i == selected { '>' } else { ' ' };
            let _ = mvprintw(i as i32 + 1, 0, &format!("{} {}", ch, name));
        }
        refresh();

        let input = getch();
        match input {
            KEY_UP => selected = (selected + _games.len() - 1) % _games.len(),
            KEY_DOWN => selected = (selected + 1) % _games.len(),
            10 => {
                clear();
                endwin();
                (_games[selected].1)();
                initscr();
                noecho();
                curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
                keypad(stdscr(), true);
                timeout(100);
            }
            _ => (),
        }
    }
}
