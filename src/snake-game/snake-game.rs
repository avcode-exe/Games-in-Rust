extern crate ncurses;
extern crate rand;
use ncurses::*;
use std::thread::sleep;
use std::time::Duration;
mod utils;
use utils::{Direction, SnakeGame};

const WIDTH: i32 = 100;
const HEIGHT: i32 = 15;

/// Play a game of Snake.
///
/// This function will initialize the ncurses library and then play a game of Snake.
/// The game will continue until the player presses the 'q' key to quit.
///
/// # Returns
/// 
/// This function does not return any value.
pub fn snake_game() -> () {
    // Initialize ncurses
    initscr();

    // Do not echo the keys pressed when playing the game
    noecho();

    // Make the cursor invisible
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Enable reading of special keys such as arrow keys
    keypad(stdscr(), true);

    // Set the timeout to 100ms
    timeout(100);

    // Create a new game of Snake
    let mut snake_game: SnakeGame = SnakeGame::new(WIDTH, HEIGHT);

    // Play the game until the player presses the 'q' key to quit
    loop {
        // Draw the current state of the game
        snake_game.draw();

        // Get the user's input
        let user_input: i32 = getch();

        // If the user pressed the 'q' key, break out of the loop
        if user_input == 'q' as i32 {
            break;
        }

        // If the user pressed a direction key, change the snake's direction
        match user_input {
            KEY_UP | KEY_DOWN | KEY_LEFT | KEY_RIGHT => {
                let dir = match user_input {
                    KEY_UP => Direction::Up,
                    KEY_DOWN => Direction::Down,
                    KEY_LEFT => Direction::Left,
                    KEY_RIGHT => Direction::Right,
                    _ => unreachable!(),
                };
                snake_game.change_direction(dir);
            }
            _ => (),
        }

        // Update the game state
        if !snake_game.update() {
            break;
        }

        // Pause for a short while before continuing
        sleep(Duration::from_millis(150));
    }

    clear();
    mvprintw(HEIGHT / 2, WIDTH / 2 - 5, "Game Over!");
    refresh();
    napms(2000);

    // Clean up ncurses
    endwin();
}
