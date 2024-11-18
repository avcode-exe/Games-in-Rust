extern crate ncurses;
extern crate rand;
use ncurses::*;
use std::cmp::min;
use std::collections::HashSet;
mod utils;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

/// Draws the current state of the game to the screen.
///
/// # Arguments
///
/// * `minefield`: A reference to the game's minefield.
/// * `revealed`: A `HashSet` containing coordinates of the cells that have been revealed.
/// * `flagged`: A `HashSet` containing coordinates of the cells that have been flagged as mines.
/// * `cursor_x`: The x coordinate of the cursor.
/// * `cursor_y`: The y coordinate of the cursor.
fn draw_game_state(
    minefield: &utils::Minefield,
    revealed: &HashSet<(usize, usize)>,
    flagged: &HashSet<(usize, usize)>,
    cursor_x: usize,
    cursor_y: usize,
) {
    // Iterate over each row of the game grid.
    for y in 0..HEIGHT {
        // Iterate over each column in the current row.
        for x in 0..WIDTH {
            // Initialize color_pair to 0, meaning no special color.
            let mut color_pair: i16 = 0;
            
            // Determine the character to display at (x, y).
            let ch = if cursor_x == x && cursor_y == y {
                // If the cursor is positioned over this cell.
                '#'
            } else if flagged.contains(&(y, x)) {
                // If this cell is flagged as a mine.
                color_pair = 10; // Use color pair for flags.
                'F'
            } else if revealed.contains(&(y, x)) {
                // If this cell has been revealed.
                if minefield.grid.contains(&(x, y)) {
                    // If the revealed cell is a mine.
                    color_pair = 1; // Use color pair for mines.
                    '*'
                } else {
                    // If the revealed cell is not a mine.
                    // Count the number of adjacent mines.
                    let count: u32 = utils::count_adjacent_mines(&minefield, x, y).into();
                    color_pair = count as i16 + 1; // Use color pair based on count.
                    // Display the number of adjacent mines.
                    std::char::from_digit(count, 10).unwrap_or(' ')
                }
            } else {
                // If the cell has not been revealed yet.
                '.'
            };

            // Apply the color pair if it is set.
            if color_pair != 0 {
                attron(COLOR_PAIR(color_pair));
            }
            
            // Move to the position (x, y) on the screen and add the character.
            mvaddch(y as i32, x as i32, ch as u32);
            
            // Turn off the current color pair if it was set.
            if color_pair != 0 {
                attroff(COLOR_PAIR(color_pair));
            }
        }
    }
}

/// Play a game of Minesweeper
///
/// # Arguments
/// * `width`: The width of the minefield.
/// * `height`: The height of the minefield.
/// * `mines`: The number of mines in the minefield.
pub fn minesweeper(width: usize, height: usize, mines: usize) {
    // Create the minefield.
    let minefield = utils::generate_minefield(width, height, mines);
    // The set of cells that have been revealed.
    let mut revealed = HashSet::<(usize, usize)>::with_capacity((width * height) as usize);
    // The set of cells that have been flagged as mines.
    let mut flagged = HashSet::<(usize, usize)>::with_capacity(mines);

    // Initialize the ncurses library.
    initscr(); // Initialize the ncurses library.
    noecho(); // Do not echo the keys pressed.
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Make the cursor invisible.
    keypad(stdscr(), true); // Enable reading of special keys such as arrow keys.
    timeout(100); // Set the timeout to 100ms.

    // Initialize colors if supported.
    if has_colors() {
        start_color(); // Initialize colors if supported.
        init_pair(
            1,
            COLOR_RED,
            COLOR_BLACK,
        ); // Color pair for mines.
        init_pair(
            2,
            COLOR_GREEN,
            COLOR_BLACK,
        ); // Color pair for count of 1 adjacent mine.
        init_pair(
            3,
            COLOR_YELLOW,
            COLOR_BLACK,
        ); // Color pair for count of 2 adjacent mines.
        init_pair(
            4,
            COLOR_BLUE,
            COLOR_BLACK,
        ); // Color pair for count of 3 adjacent mines.
        init_pair(
            5,
            COLOR_MAGENTA,
            COLOR_BLACK,
        ); // Color pair for count of 4 adjacent mines.
        init_pair(
            6,
            COLOR_CYAN,
            COLOR_BLACK,
        ); // Color pair for count of 5 adjacent mines.
        init_pair(
            7,
            COLOR_WHITE,
            COLOR_BLACK,
        ); // Color pair for count of 6 adjacent mines.
        init_pair(
            8,
            COLOR_BLACK,
            COLOR_WHITE,
        ); // Color pair for count of 7 adjacent mines.
        init_pair(
            9,
            COLOR_BLACK,
            COLOR_RED,
        ); // Color pair for count of 8 adjacent mines.
        init_pair(
            10,
            COLOR_RED,
            COLOR_WHITE,
        ); // Color pair for flags.
    }

    // Initialize the position of the cursor.
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    // The loop that controls the game.
    loop {
        // Clear the screen.
        clear();

        // Draw the game state.
        draw_game_state(
            &minefield,
            &revealed,
            &flagged,
            cursor_x,
            cursor_y,
        );

        // Refresh the screen.
        refresh();

        // Read the input from the user.
        let input = getch();

        // If the user pressed q, break the loop.
        if input == 'q' as i32 {
            break;
        }

        // Move the cursor based on the input.
        match input {
            KEY_UP => cursor_y = cursor_y.saturating_sub(1),
            KEY_DOWN => cursor_y = min(cursor_y + 1, height - 1),
            KEY_LEFT => cursor_x = cursor_x.saturating_sub(1),
            KEY_RIGHT => cursor_x = min(cursor_x + 1, width - 1),
            10 => {
                if !flagged.contains(&(cursor_y, cursor_x)) && !revealed.contains(&(cursor_y, cursor_x)) {
                    // If the user pressed Enter, reveal the cell at the current position.
                    utils::reveal_adjacent_zeros(
                        &minefield,
                        &mut revealed,
                        &flagged,
                        cursor_x,
                        cursor_y,
                    );

                    // Check if the user has pressed a mine.
                    if minefield.grid.contains(&(cursor_x, cursor_y)) {
                        // If the user has pressed a mine, end the game.
                        let _ = mvprintw(
                            height as i32 + 1,
                            0,
                            "Game Over!",
                        );
                        refresh();
                        for y in 0..height {
                            for x in 0..width {
                                if minefield.grid.contains(&(x, y)) {
                                    attron(COLOR_PAIR(1));
                                    mvaddch(y as i32, x as i32, '*' as u32);
                                    attroff(COLOR_PAIR(1));
                                } else if revealed.contains(&(y, x)) {
                                    let count = utils::count_adjacent_mines(&minefield, x, y);
                                    let ch = std::char::from_digit(count as u32, 10).unwrap_or(' ');
                                    attron(COLOR_PAIR(count as i16 + 1));
                                    mvaddch(y as i32, x as i32, ch as u32);
                                    attroff(COLOR_PAIR(count as i16 + 1));
                                }
                            }
                        }
                        refresh();
                        napms(2000);
                        break;
                    }
                }
            }
            32 => {
                // If the user pressed the Space key, toggle the flag on the cell at the current position.
                if !revealed.contains(&(cursor_y, cursor_x)) {
                    if flagged.contains(&(cursor_y, cursor_x)) {
                        flagged.retain(|&(y, x)| !(y == cursor_y && x == cursor_x));
                    } else {
                        flagged.insert((cursor_y, cursor_x));
                    }
                }
            }
            _ => (),
        }

        // Check if the user has won the game.
        let won = (0..height)
            .all(|y| (0..width).all(|x| minefield.grid.contains(&(x, y)) || revealed.contains(&(y, x))));

        if won {
            // If the user has won, end the game.
            let _ = mvprintw(height as i32 + 1, 0, "You Won!");
            refresh();
            napms(2000);
            break;
        }
    }

    // Clean up the ncurses library.
    endwin();
}
