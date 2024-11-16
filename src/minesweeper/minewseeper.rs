extern crate ncurses;
extern crate rand;
use ncurses::*;
use std::cmp::min;
mod utils;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const MINES: usize = 10;

fn draw_game_state(
    minefield: &utils::Minefield,
    revealed: &Vec<Vec<bool>>,
    flagged: &Vec<Vec<bool>>,
    cursor_x: usize,
    cursor_y: usize,
) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ch = if cursor_x == x && cursor_y == y {
                '#'
            } else if flagged[y][x] {
                attron(COLOR_PAIR(10));
                'F'
            } else if revealed[y][x] {
                if minefield.grid[y][x] {
                    attron(COLOR_PAIR(1));
                    '*'
                } else {
                    let count = utils::count_adjacent_mines(&minefield, x, y);
                    attron(COLOR_PAIR(count as i16 + 1));
                    std::char::from_digit(count as u32, 10).unwrap_or(' ')
                }
            } else {
                '.'
            };
            mvaddch(y as i32, x as i32, ch as u32);
            attroff(COLOR_PAIR(1));
            attroff(COLOR_PAIR(2));
            attroff(COLOR_PAIR(3));
            attroff(COLOR_PAIR(4));
            attroff(COLOR_PAIR(5));
            attroff(COLOR_PAIR(6));
            attroff(COLOR_PAIR(7));
            attroff(COLOR_PAIR(8));
            attroff(COLOR_PAIR(9));
            attroff(COLOR_PAIR(10));
        }
    }
}

pub fn minesweeper() {
    let minefield = utils::generate_minefield(WIDTH, HEIGHT, MINES);
    let mut revealed = vec![vec![false; WIDTH]; HEIGHT];
    let mut flagged = vec![vec![false; WIDTH]; HEIGHT];
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    timeout(100);

    if has_colors() {
        start_color();
        init_pair(1, COLOR_RED, COLOR_BLACK); // Mines
        init_pair(2, COLOR_GREEN, COLOR_BLACK); // 1
        init_pair(3, COLOR_YELLOW, COLOR_BLACK); // 2
        init_pair(4, COLOR_BLUE, COLOR_BLACK); // 3
        init_pair(5, COLOR_MAGENTA, COLOR_BLACK); // 4
        init_pair(6, COLOR_CYAN, COLOR_BLACK); // 5
        init_pair(7, COLOR_WHITE, COLOR_BLACK); // 6
        init_pair(8, COLOR_BLACK, COLOR_WHITE); // 7
        init_pair(9, COLOR_BLACK, COLOR_RED); // 8
        init_pair(10, COLOR_RED, COLOR_WHITE); // Flags
    }

    loop {
        clear();
        draw_game_state(&minefield, &revealed, &flagged, cursor_x, cursor_y);
        refresh();

        let input = getch();

        if input == 'q' as i32 {
            break;
        }

        match input {
            KEY_UP => cursor_y = cursor_y.saturating_sub(1),
            KEY_DOWN => cursor_y = min(cursor_y + 1, HEIGHT - 1),
            KEY_LEFT => cursor_x = cursor_x.saturating_sub(1),
            KEY_RIGHT => cursor_x = min(cursor_x + 1, WIDTH - 1),
            10 => {
                if !flagged[cursor_y][cursor_x] && !revealed[cursor_y][cursor_x] {
                    utils::reveal_adjacent_zeros(
                        &minefield,
                        &mut revealed,
                        &flagged,
                        cursor_x,
                        cursor_y,
                    );
                    if minefield.grid[cursor_y][cursor_x] {
                        let _ = mvprintw(HEIGHT as i32 + 1, 0, "Game Over!");
                        refresh();
                        for y in 0..HEIGHT {
                            for x in 0..WIDTH {
                                if minefield.grid[y][x] {
                                    attron(COLOR_PAIR(1));
                                    mvaddch(y as i32, x as i32, '*' as u32);
                                    attroff(COLOR_PAIR(1));
                                } else {
                                    let count = utils::count_adjacent_mines(&minefield, x, y);
                                    let ch = if flagged[y][x] && !minefield.grid[y][x] {
                                        'X'
                                    } else {
                                        std::char::from_digit(count as u32, 10).unwrap_or(' ')
                                    };
                                    attron(COLOR_PAIR(count as i16 + 1));
                                    mvaddch(y as i32, x as i32, ch as u32);
                                    attroff(COLOR_PAIR(count as i16 + 1));
                                }
                            }
                        }
                        refresh();
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        break;
                    }
                }
            }
            32 => {
                if !revealed[cursor_y][cursor_x] {
                    flagged[cursor_y][cursor_x] = !flagged[cursor_y][cursor_x];
                }
            }
            _ => (),
        }

        // Check if the player has won
        let won = (0..HEIGHT).all(|y| (0..WIDTH).all(|x| minefield.grid[y][x] || revealed[y][x]));

        if won {
            let _ = mvprintw(HEIGHT as i32 + 1, 0, "You Won!");
            refresh();
            std::thread::sleep(std::time::Duration::from_secs(2));
            break;
        }
    }

    endwin();
}
