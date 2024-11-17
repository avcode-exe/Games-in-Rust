extern crate ncurses;
extern crate rand;
use ncurses::*;
use rand::seq::SliceRandom;
use std::collections::HashSet;

const MAZE_WIDTH: i32 = 31;
const MAZE_HEIGHT: i32 = 15;

pub struct Maze {
    pub grid: HashSet<(i32, i32)>,
}

pub fn generate_maze(rng: &mut impl rand::Rng) -> Maze {
    let mut grid = HashSet::new();

    fn carve_passage(grid: &mut HashSet<(i32, i32)>, x: i32, y: i32, rng: &mut impl rand::Rng) {
        grid.insert((x, y));

        let mut directions = [(0, 2), (2, 0), (0, -2), (-2, 0)].to_vec();
        directions.shuffle(rng);

        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;

            if nx > 0 && nx < MAZE_WIDTH - 1 && ny > 0 && ny < MAZE_HEIGHT - 1 && !grid.contains(&(nx, ny)) {
                if (y + dy / 2) >= 0 && (y + dy / 2) < MAZE_HEIGHT && (x + dx / 2) >= 0 && (x + dx / 2) < MAZE_WIDTH {
                    grid.insert((x + dx / 2, y + dy / 2));
                    carve_passage(grid, nx, ny, rng);
                }
            }
        }
    }

    carve_passage(&mut grid, 1, 1, rng);
    Maze { grid }
}

pub fn draw_maze(maze: &Maze) {
    for y in 0..MAZE_HEIGHT {
        for x in 0..MAZE_WIDTH {
            let ch = if maze.grid.contains(&(x, y)) { ' ' } else { '#' };
            mvaddch(y, x, ch as u32);
        }
    }
}