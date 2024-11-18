extern crate ncurses;
extern crate rand;

use ncurses::*;
use std::process;

#[path = "maze/maze.rs"]
mod maze;
#[path = "minesweeper/minesweeper.rs"]
mod minesweeper;
#[path = "snake-game/snake-game.rs"]
mod snake_game;

// Define a Game type to encapsulate game data
struct Game {
    name: String,
    func: fn() -> Result<(), Box<dyn std::error::Error>>, // Function with error handling
}

/// The main entry point of the application.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a vector of Games, using a struct for better organization.
    // The `Game` struct encapsulates the name of the game and a function to
    // execute the game.
    let games = vec![
        Game {
            name: "Maze".to_string(),
            func: || { maze::maze(); Ok(()) }, // Call the maze::maze function
        },
        Game {
            name: "Minesweeper".to_string(),
            func: || { minesweeper::minesweeper(10, 10, 10); Ok(()) }, // Call the minesweeper::minesweeper function
        },
        Game {
            name: "Snake".to_string(),
            func: || { snake_game::snake_game(); Ok(()) }, // Call the snake_game::snake_game function
        },
        Game {
            name: "Quit".to_string(),
            func: || Ok(()), // Return Ok(()) to match other functions
        },
    ];

    // Initialize the ncurses library, which provides a terminal-independent
    // way of interacting with the user in a console.
    initscr();
    noecho(); // Do not echo the keys pressed.
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Make the cursor invisible.
    keypad(stdscr(), true); // Enable reading of special keys such as arrow keys.
    timeout(100); // Set the timeout to 100ms.

    let mut selected = 0; // Initialize the selected index to 0.

    loop {
        clear(); // Clear the screen.
        mvprintw(0, 0, "Select a game or quit:"); // Print a message at the top of the screen.

        // Iterate over the vector of Games and print each game name.
        for (i, game) in games.iter().enumerate() {
            let ch = if i == selected { '>' } else { ' ' }; // Determine whether to print a '>' or ' ' character.
            mvprintw(i as i32 + 1, 0, &format!("{} {}", ch, game.name)); // Print the character and game name.
        }

        refresh(); // Update the display.

        let input = getch(); // Get the character that was pressed.

        match input {
            KEY_UP => {
                // If the UP arrow key was pressed, increment the selected index.
                selected = (selected + games.len() - 1) % games.len();
            }
            KEY_DOWN => {
                // If the DOWN arrow key was pressed, decrement the selected index.
                selected = (selected + 1) % games.len();
            }
            10 => {
                // If the Enter key was pressed, execute the selected game.
                clear(); // Clear the screen.
                endwin(); // End the ncurses mode.
                if let Err(e) = (games[selected].func)() {
                    // Handle game execution and errors.
                    // Print an error message if the game execution failed.
                    eprintln!("Error during game: {}", e);
                    // Go back to the menu.
                    initscr();
                    noecho();
                    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
                    keypad(stdscr(), true);
                    timeout(100);
                    continue; // Continue loop after error
                } else if games[selected].name == "Quit" {
                    // If the selected game is "Quit", exit the program.
                    process::exit(0);
                }

                // If the game execution was successful, go back to the menu.
                initscr();
                noecho();
                curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
                keypad(stdscr(), true);
                timeout(100);
            }
            _ => (), // Ignore all other characters.
        }
    }
}
