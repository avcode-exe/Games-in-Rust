extern crate rand;
use rand::Rng;
use std::collections::{HashSet, VecDeque};
use ncurses::*;

const WIDTH: i32 = 100;
const HEIGHT: i32 = 15;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeGame {
    snake: VecDeque<(i32, i32)>,
    snake_set: HashSet<(i32, i32)>,
    direction: Direction,
    food: (i32, i32),
    score: i32,
}

impl SnakeGame {
    /// Creates a new instance of `SnakeGame`.
    ///
    /// This function initializes a new game with the snake starting in the middle of the
    /// game area, a random food position, and the initial direction set to `Right`.
    ///
    /// # Returns
    ///
    /// A new `SnakeGame` instance.
    pub fn new(width: i32, height: i32) -> SnakeGame {
        // Create a new thread-local random number generator.
        let mut rng = rand::thread_rng();

        // Generate a random number between 1 and the width of the game area for the x coordinate
        // of the food.
        let food_x = rng.gen_range(1..width - 1);

        // Generate a random number between 1 and the height of the game area for the y coordinate
        // of the food.
        let food_y = rng.gen_range(1..height - 1);

        // Calculate the center of the game area for the x and y coordinates.
        let center_x = width / 2;
        let center_y = height / 2;

        // Create a new `SnakeGame` instance with the snake starting in the middle of the game
        // area, the food in a random position, the initial direction set to `Right`, and a
        // score of 0.
        SnakeGame {
            snake: vec![(center_x, center_y)].into_iter().collect(),
            snake_set: std::iter::once((center_x, center_y)).collect(),
            direction: Direction::Right,
            food: (food_x, food_y),
            score: 0,
        }
    }

    /// Draws the current state of the snake game to the screen.
    ///
    /// This function is responsible for drawing the boundaries of the game area,
    /// the snake, the food, and the score to the screen. It will clear the screen
    /// first to remove the previous state of the game, then draw the game
    /// boundaries, the snake, the food, and the score. Finally, it will refresh
    /// the screen to reflect all changes made.
    ///
    /// # Arguments
    ///
    /// * `self`: A reference to the `SnakeGame` instance.
    ///
    /// # Returns
    ///
    /// `()` (an empty tuple).
    pub fn draw(&self) -> () {
        // Clear the screen to start drawing the updated game state.
        clear();

        // Draw the game boundaries.
        // The top and bottom boundaries are drawn as a horizontal line of '#' characters.
        // The left and right boundaries are drawn as a vertical line of '#' characters.
        mvhline(0, 0, '#' as u32, WIDTH as i32); // Top boundary
        mvhline(HEIGHT - 1, 0, '#' as u32, WIDTH as i32); // Bottom boundary
        mvvline(0, 0, '#' as u32, HEIGHT as i32); // Left boundary
        mvvline(0, WIDTH - 1, '#' as u32, HEIGHT as i32); // Right boundary

        // Draw the snake on the screen.
        // For each segment of the snake, draw a '#' character at the segment's position.
        for &(x, y) in &self.snake {
            mvaddch(y, x, '#' as u32); // Snake body
        }

        // Draw the food on the screen.
        // Draw a '*' character at the food's position.
        mvaddch(self.food.1, self.food.0, '*' as u32); // Food

        // Display the score below the game area.
        // Format the score as a string and display it below the game area.
        mvprintw(HEIGHT, 0, &format!("Score: {}", self.score));

        // Refresh the screen to reflect all changes made.
        // This will update the screen with all the changes made by the above code.
        refresh();
    }

    /// Updates the game state by moving the snake in the current direction.
    ///
    /// This function is responsible for moving the snake's head in the current direction 
    /// and updating the snake's body accordingly. It checks for collisions with the walls 
    /// and the snake's own body. If a collision occurs, the game ends. If the snake eats 
    /// the food, the score is incremented, and new food is placed randomly on the board.
    ///
    /// # Returns
    ///
    /// Returns `false` if the game is over due to a collision, otherwise returns `true`.
    pub fn update(&mut self) -> bool {

        // Calculate the new head position based on the current direction of the snake.
        // Direction::Up means moving up, which decrements the y-coordinate.
        // Direction::Down means moving down, which increments the y-coordinate.
        // Direction::Left means moving left, which decrements the x-coordinate.
        // Direction::Right means moving right, which increments the x-coordinate.
        let new_head = match self.direction {
            Direction::Up => (self.snake[0].0, self.snake[0].1 - 1),
            Direction::Down => (self.snake[0].0, self.snake[0].1 + 1),
            Direction::Left => (self.snake[0].0 - 1, self.snake[0].1),
            Direction::Right => (self.snake[0].0 + 1, self.snake[0].1),
        };

        // Check if the new head position causes a collision with the snake's body or the walls.
        // The function is_collision checks for these conditions.
        if self.is_collision(&new_head) {
            return false; // Collision detected; game over.
        }

        // Insert the new head position into the snake_set to keep track of the snake's body positions.
        self.snake_set.insert(new_head);

        // Push the new head position to the front of the snake, effectively moving the snake forward.
        self.snake.push_front(new_head);

        // Check if the new head position matches the food position.
        if new_head == self.food {
            // If the snake eats the food, increment the score.
            self.score += 1;

            // Generate new food position randomly on the board.
            self.food = self.generate_food();
        } else {
            // If the snake did not eat the food, remove the snake's tail to maintain the same length.
            // Remove the last element from the snake's deque and its position from the snake_set.
            self.snake_set.remove(&self.snake.pop_back().unwrap());
        }

        true // Return true indicating the game continues.
    }

    /// Checks if a given position is out of the game area or if the snake has collided with its own body.
    ///
    /// This function checks if a given position is out of the game area or if the snake has collided with its own body.
    /// The function takes two arguments: a reference to the `SnakeGame` instance and a tuple (x, y) representing the position to check for collision.
    /// The function first checks if the position is out of the game area, then checks if the position is in the snake's body.
    /// The function returns `true` if the position is out of the game area or if the snake has collided with its own body, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `self`: A reference to the `SnakeGame` instance.
    /// * `position`: A tuple (x, y) representing the position to check for collision.
    ///
    /// # Returns
    ///
    /// Returns `true` if the position is out of the game area or if the snake has collided with its own body, `false` otherwise.
    fn is_collision(&self, position: &(i32, i32)) -> bool {
        let (x, y) = position;
        // Check if the position is out of the game area.
        // If the position is out of the game area, the snake has collided with the wall and the game is over.
        // The game area is bounded by the left and right edges of the game board, and the top and bottom edges of the game board.
        // The left edge is at x = 0, the right edge is at x = WIDTH - 1, the top edge is at y = 0, and the bottom edge is at y = HEIGHT - 1.
        x <= &0 || x >= &(WIDTH - 1) || y <= &0 || y >= &(HEIGHT - 1)
            // Check if the position is in the snake's body.
            // If the position is in the snake's body, the snake has collided with itself and the game is over.
            // The snake's body is represented by the snake_set, which is a HashSet of (x, y) positions.
            // The position is in the snake's body if it is present in the snake_set.
            || self.snake_set.contains(position)
    }

    /// Generates a random food position on the board.
    ///
    /// This function generates a random position for the food on the board. The position is
    /// chosen randomly from the range of valid positions on the board, which is from 1 to
    /// WIDTH - 1 for the x coordinate and from 1 to HEIGHT - 1 for the y coordinate.
    ///
    /// # Returns
    ///
    /// A tuple `(i32, i32)` representing the generated food position.
    fn generate_food(&self) -> (i32, i32) {
        // Generate a list of all valid positions on the board. The list is constructed by
        // taking the range of valid x coordinates and the range of valid y coordinates and
        // using the `flat_map` method to create an iterator over the product of the two
        // ranges. The iterator will generate a tuple `(x, y)` for each valid position.
        let mut positions = (1..WIDTH - 1).flat_map(|x| (1..HEIGHT - 1).map(move |y| (x, y)));

        // Find the first position in the list that is not in the snake's body. This is done
        // by using the `find` method on the iterator and providing a closure that takes a
        // reference to a tuple `(x, y)` and checks if the position is not in the snake's
        // body. If the position is not in the snake's body, the closure will return `true`
        // and the `find` method will return the position. If the position is in the snake's
        // body, the closure will return `false` and the `find` method will skip the
        // position and continue searching for the next valid position.
        positions.find(|&p| !self.snake_set.contains(&p)).unwrap()
    }

    /// Changes the direction of the snake.
    ///
    /// This function takes a mutable reference to the `SnakeGame` instance and a new
    /// direction as a `Direction` enum. It updates the snake's direction based on the
    /// provided new direction, but only if the new direction is not the opposite of
    /// the current direction.
    ///
    /// For example, if the snake is moving up, it cannot change its direction to
    /// down directly. This is to prevent the snake from reversing its direction
    /// instantly.
    ///
    /// The function works by first finding the opposite direction of the current
    /// direction. It does this by matching the current direction against all
    /// possible directions and returning the opposite direction.
    ///
    /// Then, it checks if the new direction is not the opposite direction. If it is
    /// not, then the function updates the direction of the snake to the new
    /// direction.
    ///
    /// If the new direction is the opposite direction, the function does not update
    /// the direction and simply returns without doing anything.
    pub fn change_direction(&mut self, new_direction: Direction) -> () {
        // Find the opposite direction of the current direction
        let opposite_direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        // Update the direction only if the new direction is not the opposite
        // direction
        if new_direction != opposite_direction {
            self.direction = new_direction;
        }
    }
}
