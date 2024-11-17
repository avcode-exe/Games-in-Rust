extern crate ncurses;
extern crate rand;
use rand::Rng;
use std::collections::HashSet;

pub struct Minefield {
    pub grid: HashSet<(usize, usize)>,
    pub width: usize,
    pub height: usize,
}

/// Generates a `Minefield` with the specified dimensions and number of mines.
///
/// This function takes the width and height of the minefield as well as the number of
/// mines to place in the minefield. It will then randomly place the specified number
/// of mines in the minefield and return a `Minefield` struct containing the grid with
/// the mines.
///
/// # Arguments
///
/// * `width: usize` - The width of the minefield.
/// * `height: usize` - The height of the minefield.
/// * `mines: usize` - The number of mines to place in the minefield.
///
/// # Returns
///
/// A `Minefield` struct containing a grid with randomly placed mines.
pub fn generate_minefield(
    width: usize,
    height: usize,
    mines: usize,
) -> Minefield {
    // Create a new empty HashSet to store the positions of the mines.
    let mut mine_positions = HashSet::new();

    // Create a new thread-local random number generator.
    let mut rng = rand::thread_rng();

    // Loop until the specified number of mines have been placed.
    while mine_positions.len() < mines {
        // Generate a random number between 0 and the width of the minefield.
        let x = rng.gen_range(0..width);

        // Generate a random number between 0 and the height of the minefield.
        let y = rng.gen_range(0..height);

        // Insert the randomly generated coordinates into the HashSet.
        // If the coordinates already exist in the HashSet, the insert will fail,
        // but that's okay because we'll just try again.
        mine_positions.insert((x, y));
    }

    // Create a new `Minefield` struct containing the grid with the randomly placed
    // mines.
    Minefield { grid: mine_positions, width, height }
}

/// Counts the number of adjacent mines to a given cell in the minefield.
///
/// # Arguments
/// * `minefield: &Minefield` - A reference to the minefield.
/// * `cell_x: usize` - The x coordinate of the cell.
/// * `cell_y: usize` - The y coordinate of the cell.
///
/// # Returns
/// The number of mines adjacent to the specified cell.
///
/// This function works by iterating over the 8 possible adjacent cells to the
/// given cell. For each adjacent cell, it checks if the cell is a mine by looking
/// it up in the `minefield`'s `grid` HashSet. If the cell is a mine, it increments
/// the count.
pub fn count_adjacent_mines(minefield: &Minefield, cell_x: usize, cell_y: usize) -> u8 {
    let mut count = 0;

    // Iterate over the 8 possible adjacent cells
    for &(dx, dy) in &[
        // Top-left
        (-1, -1),
        // Top
        (-1, 0),
        // Top-right
        (-1, 1),
        // Left
        (0, -1),
        // Right
        (0, 1),
        // Bottom-left
        (1, -1),
        // Bottom
        (1, 0),
        // Bottom-right
        (1, 1),
    ] {
        let nx = cell_x as isize + dx;
        let ny = cell_y as isize + dy;

        // Check if the adjacent cell is within the bounds of the minefield.
        if nx >= 0
            && ny >= 0
            && nx < minefield.width as isize
            && ny < minefield.height as isize
        {
            // If the adjacent cell is within the bounds, check if it's a mine.
            if minefield.grid.contains(&(nx as usize, ny as usize)) {
                // If the adjacent cell is a mine, increment the count.
                count += 1;
            }
        }
    }

    // Return the count of adjacent mines.
    count
}

/// Reveal all adjacent cells that are not mines if the cell at position (x, y) has no
/// adjacent mines or if all adjacent mines have been flagged.
///
/// # Arguments
/// * `minefield: &Minefield` - A reference to the minefield.
/// * `revealed: &mut HashSet<(usize, usize)>` - The set of revealed cells.
/// * `flagged: &HashSet<(usize, usize)>` - The set of flagged cells.
/// * `x: usize` - The x coordinate of the cell.
/// * `y: usize` - The y coordinate of the cell.
pub fn reveal_adjacent_zeros(
    minefield: &Minefield,
    revealed: &mut HashSet<(usize, usize)>,
    flagged: &HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    // If the cell at (y, x) is already revealed, return immediately.
    if revealed.contains(&(y, x)) {
        return;
    }

    // Add the current cell (y, x) to the set of revealed cells.
    revealed.insert((y, x));

    // Count the number of mines adjacent to the current cell.
    let adjacent_mines = count_adjacent_mines(minefield, x, y);

    // If there are no adjacent mines, reveal all adjacent cells.
    if adjacent_mines == 0 {
        // Iterate over all possible directions (dx, dy) to adjacent cells.
        for &(dx, dy) in &[
            (-1, -1), // Top-left
            (-1, 0),  // Top
            (-1, 1),  // Top-right
            (0, -1),  // Left
            (0, 1),   // Right
            (1, -1),  // Bottom-left
            (1, 0),   // Bottom
            (1, 1),   // Bottom-right
        ] {
            // Calculate the coordinates (nx, ny) of the adjacent cell.
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            // Check if the adjacent cell (nx, ny) is within the minefield bounds.
            if nx < minefield.width && ny < minefield.height {
                // Recursively reveal the adjacent cell.
                reveal_adjacent_zeros(minefield, revealed, flagged, nx, ny);
            }
        }
    } else {
        // Count the number of flagged cells adjacent to the current cell.
        let flagged_count = (0..8)
            .filter(|&i| flagged.contains(&((y as isize + DY[i]) as usize, (x as isize + DX[i]) as usize)))
            .count() as u8;

        // If the number of flagged cells equals the number of adjacent mines,
        // reveal all non-mine adjacent cells.
        if flagged_count == adjacent_mines {
            // Iterate over all possible directions (dx, dy) to adjacent cells.
            for i in 0..8 {
                // Calculate the coordinates (nx, ny) of the adjacent cell.
                let nx = (x as isize + DX[i]) as usize;
                let ny = (y as isize + DY[i]) as usize;

                // Check if the adjacent cell (nx, ny) is within the minefield bounds
                // and is not a mine.
                if nx < minefield.width && ny < minefield.height && !minefield.grid.contains(&(ny, nx)) {
                    // Recursively reveal the adjacent cell.
                    reveal_adjacent_zeros(minefield, revealed, flagged, nx, ny);
                }
            }
        }
    }
}

static DX: [isize; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
static DY: [isize; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
