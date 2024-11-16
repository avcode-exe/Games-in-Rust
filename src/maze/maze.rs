extern crate ncurses;
extern crate serde;
extern crate rand;
use ncurses::*;
mod utils;

const MAZE_WIDTH: i32 = 31;
const MAZE_HEIGHT: i32 = 15;

pub fn maze() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(100);

    let mut rng = rand::thread_rng();
    let maze = utils::generate_maze(&mut rng);
    let mut player_x = 1;
    let mut player_y = 1;
    let target_x = MAZE_WIDTH - 2;
    let target_y = MAZE_HEIGHT - 2;

    loop {
        clear();
        utils::draw_maze(&maze);
        mvaddch(player_y, player_x, '@' as u32);
        mvaddch(target_y, target_x, 'X' as u32);
        refresh();

        let input = getch();

        if input == 'q' as i32 {
            break;
        }

        let (next_x, next_y) = match input {
            KEY_UP => (player_x, player_y - 1),
            KEY_DOWN => (player_x, player_y + 1),
            KEY_LEFT => (player_x - 1, player_y),
            KEY_RIGHT => (player_x + 1, player_y),
            _ => (player_x, player_y),
        };

        if next_x >= 0 && next_x < MAZE_WIDTH && next_y >= 0 && next_y < MAZE_HEIGHT && maze.grid[next_y as usize][next_x as usize] == 0 {
            player_x = next_x;
            player_y = next_y;
        }

        if player_x == target_x && player_y == target_y {
            clear();
            let _ = mvprintw(MAZE_HEIGHT / 2, MAZE_WIDTH / 2 - 5, "You Win!");
            refresh();
            std::thread::sleep(std::time::Duration::from_secs(2));
            break;
        }
    }

    endwin();
}