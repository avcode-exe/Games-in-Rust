extern crate ncurses;
extern crate rand;
use ncurses::*;
mod utils;

const MAZE_WIDTH: i32 = 31;
const MAZE_HEIGHT: i32 = 15;

/*************  ✨ Codeium Command 🌟  *************/
/// Draws the current state of the maze game to the screen.
/// 
/// This function refreshes the screen to display the maze along with the player's
/// and target's positions. The player is represented by the '@' character, and the
/// target is represented by the 'X' character. The function makes use of the ncurses
/// library to handle terminal graphics.
///
///
/// * `maze`: A reference to the maze structure that contains the layout of the maze.
/// * `player_x`: The column index representing the player's current position.
/// * `player_y`: The row index representing the player's current position.
/// * `target_x`: The column index representing the target's position.
/// * `target_y`: The row index representing the target's position.
fn draw_game_state(
    maze: &utils::Maze,
    player_x: i32,
    player_y: i32,
    target_x: i32,
    target_y: i32,
) -> () {
    // Clear the screen to start drawing the updated maze state.
    clear();

    // Draw the maze structure on the screen.
    // This function is expected to use the ncurses library to render the maze.
    utils::draw_maze(maze);

    // Place the player character '@' on the screen at the player's current position.
    // The mvaddch function moves the cursor to the specified coordinates and adds
    // the character there.
    mvaddch(player_y, player_x, '@' as u32);

    // Place the target character 'X' on the screen at the target's position.
    mvaddch(target_y, target_x, 'X' as u32);

    // Refresh the screen to reflect all changes made, displaying the updated maze
    // with the player and target positions.
    refresh();
}

/// Returns true if the position is within the maze and is not a wall, false otherwise.
fn is_valid_position(maze: &utils::Maze, x: i32, y: i32) -> bool {
    x >= 0 && x < MAZE_WIDTH && y >= 0 && y < MAZE_HEIGHT && maze.grid.contains(&(x, y))
}

/// Play a game of Maze.
/// This function is the entry point for the Maze game. It will initialize the
/// ncurses library and display the maze and the player's and target's positions.
/// The player can move around the maze using the arrow keys. The goal is to
/// reach the target position.
/// The game will continue until the player reaches the target position or until
/// the player presses the 'q' key to quit.
/// # Arguments
///
///
pub fn maze() {
    // Check if the maze width and height are valid.
    assert!(MAZE_WIDTH > 2 && MAZE_HEIGHT > 2, "Maze width and height must be greater than 2.");

    // Initialize ncurses library.
    let window = initscr();
    noecho(); // Do not echo the keys pressed.
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Make the cursor invisible.
    keypad(window, true); // Enable reading of special keys such as arrow keys.
    timeout(100); // Set the timeout to 100ms.

    // Generate a maze.
    let mut rng = rand::thread_rng();
    let maze = utils::generate_maze(&mut rng);
    let mut player_position = (1, 1); // The player starts at position (1, 1).
    let target_position = (MAZE_WIDTH - 2, MAZE_HEIGHT - 2); // The target is at the bottom right corner of the maze.

    loop {
        // Draw the maze and the player's and target's positions.
        draw_game_state(&maze, player_position.0, player_position.1, target_position.0, target_position.1);

        // Get the player's input.
        let input = getch();

        // If the player pressed the 'q' key, quit the game.
        if input == 'q' as i32 {
            break;
        }

        // Determine the new position of the player based on the input.
        let (next_x, next_y) = match input {
            KEY_UP => (player_position.0, player_position.1 - 1), // Move up if the up arrow key is pressed.
            KEY_DOWN => (player_position.0, player_position.1 + 1), // Move down if the down arrow key is pressed.
            KEY_LEFT => (player_position.0 - 1, player_position.1), // Move left if the left arrow key is pressed.
            KEY_RIGHT => (player_position.0 + 1, player_position.1), // Move right if the right arrow key is pressed.
            _ => player_position, // Do not move if any other key is pressed.
        };

        // If the new position is within the maze and is not a wall, move the player there.
        if is_valid_position(&maze, next_x, next_y) {
            player_position = (next_x, next_y);
        }

        // If the player has reached the target position, end the game.
        if player_position == target_position {
            clear(); // Clear the screen.
            mvprintw(MAZE_HEIGHT / 2, MAZE_WIDTH / 2 - 5, "You Win!"); // Print a message to the middle of the screen indicating that the player has won.
            refresh(); // Refresh the screen to reflect the changes made.
            napms(2000); // Wait for 2 seconds before ending the game.
            break;
        }

    }

    endwin(); // Deinitialize the ncurses library.
}
