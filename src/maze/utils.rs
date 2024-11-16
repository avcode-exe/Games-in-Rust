extern crate ncurses;
extern crate rand;
extern crate serde;
use ncurses::*;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

const MAZE_WIDTH: i32 = 31;
const MAZE_HEIGHT: i32 = 15;

#[derive(Serialize, Deserialize)]
pub struct Maze {
    pub grid: Vec<Vec<i32>>,
}

pub fn generate_maze(rng: &mut impl rand::Rng) -> Maze {
    let mut maze = vec![vec![1; MAZE_WIDTH as usize]; MAZE_HEIGHT as usize];

    fn carve_passage(maze: &mut Vec<Vec<i32>>, x: i32, y: i32, rng: &mut impl rand::Rng) {
        maze[y as usize][x as usize] = 0;

        let mut directions = [(0, 2), (2, 0), (0, -2), (-2, 0)].to_vec();
        directions.shuffle(rng);

        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;

            if nx > 0 && nx < MAZE_WIDTH - 1 && ny > 0 && ny < MAZE_HEIGHT - 1 && maze[ny as usize][nx as usize] == 1 {
                if (y + dy / 2) >= 0 && (y + dy / 2) < MAZE_HEIGHT && (x + dx / 2) >= 0 && (x + dx / 2) < MAZE_WIDTH {
                    maze[(y + dy / 2) as usize][(x + dx / 2) as usize] = 0;
                    carve_passage(maze, nx, ny, rng);
                }
            }
        }
    }

    carve_passage(&mut maze, 1, 1, rng);
    Maze { grid: maze }
}

pub fn draw_maze(maze: &Maze) {
    for (y, row) in maze.grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let ch = if cell == 1 { '#' } else { ' ' };
            mvaddch(y as i32, x as i32, ch as u32);
        }
    }
}