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
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let food_x = rng.gen_range(1..WIDTH - 1);
        let food_y = rng.gen_range(1..HEIGHT - 1);
        let initial_position = (WIDTH / 2, HEIGHT / 2);

        SnakeGame {
            snake: VecDeque::from([initial_position]),
            snake_set: HashSet::from([initial_position]),
            direction: Direction::Right,
            food: (food_x, food_y),
            score: 0,
        }
    }

    pub fn draw(&self) {
        clear();
        for x in 0..WIDTH {
            mvaddch(0, x, '#' as u32);
            mvaddch(HEIGHT - 1, x, '#' as u32);
        }
        for y in 0..HEIGHT {
            mvaddch(y, 0, '#' as u32);
            mvaddch(y, WIDTH - 1, '#' as u32);
        }
        for &(x, y) in &self.snake {
            mvaddch(y, x, '#' as u32);
        }
        mvaddch(self.food.1, self.food.0, '*' as u32);
        mvprintw(HEIGHT, 0, &format!("Score: {}", self.score));
        refresh();
    }

    pub fn update(&mut self) -> bool {
        let (head_x, head_y) = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (*head_x, head_y - 1),
            Direction::Down => (*head_x, head_y + 1),
            Direction::Left => (head_x - 1, *head_y),
            Direction::Right => (head_x + 1, *head_y),
        };

        if new_head.0 <= 0 || new_head.0 >= WIDTH - 1 || new_head.1 <= 0 || new_head.1 >= HEIGHT - 1 {
            clear();
            mvprintw(HEIGHT / 2, WIDTH / 2 - 5, "Game Over!");
            refresh();
            std::thread::sleep(std::time::Duration::from_secs(2));
            return false;
        }

        if self.snake_set.contains(&new_head) {
            clear();
            mvprintw(HEIGHT / 2, WIDTH / 2 - 5, "Game Over!");
            refresh();
            std::thread::sleep(std::time::Duration::from_secs(2));
            return false;
        }

        self.snake.push_front(new_head);
        self.snake_set.insert(new_head);

        if new_head == self.food {
            self.score += 1;
            let mut rng = rand::thread_rng();
            self.food = (rng.gen_range(1..WIDTH - 1), rng.gen_range(1..HEIGHT - 1));
        } else {
            let tail = self.snake.pop_back().unwrap();
            self.snake_set.remove(&tail);
        }

        true
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if (self.direction == Direction::Up && new_direction != Direction::Down)
            || (self.direction == Direction::Down && new_direction != Direction::Up)
            || (self.direction == Direction::Left && new_direction != Direction::Right)
            || (self.direction == Direction::Right && new_direction != Direction::Left)
        {
            self.direction = new_direction;
        }
    }
}

