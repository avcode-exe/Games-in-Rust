use rand::Rng;
use std::collections::HashSet;

pub struct Minefield {
    pub grid: HashSet<(usize, usize)>,
    pub width: usize,
    pub height: usize,
}

pub fn generate_minefield(width: usize, height: usize, mines: usize) -> Minefield {
    let mut grid = HashSet::new();
    let mut rng = rand::thread_rng();

    while grid.len() < mines {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        grid.insert((x, y));
    }

    Minefield { grid, width, height }
}

pub fn count_adjacent_mines(minefield: &Minefield, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for i in (x.saturating_sub(1))..=(x + 1).min(minefield.width - 1) {
        for j in (y.saturating_sub(1))..=(y + 1).min(minefield.height - 1) {
            if minefield.grid.contains(&(i, j)) {
                count += 1;
            }
        }
    }
    count
}

pub fn reveal_adjacent_zeros(
    minefield: &Minefield,
    revealed: &mut HashSet<(usize, usize)>,
    flagged: &HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    if revealed.contains(&(y, x)) {
        return;
    }
    revealed.insert((y, x));
    let adjacent_mines = count_adjacent_mines(minefield, x, y);
    if adjacent_mines == 0 {
        for i in (x.saturating_sub(1))..=(x + 1).min(minefield.width - 1) {
            for j in (y.saturating_sub(1))..=(y + 1).min(minefield.height - 1) {
                if !(i == x && j == y) {
                    reveal_adjacent_zeros(minefield, revealed, flagged, i, j);
                }
            }
        }
    } else {
        let flagged_count = (x.saturating_sub(1)..=(x + 1).min(minefield.width - 1))
            .flat_map(|i| (y.saturating_sub(1)..=(y + 1).min(minefield.height - 1)).map(move |j| (i, j)))
            .filter(|&(i, j)| flagged.contains(&(j, i)))
            .count() as u8;
        if flagged_count == adjacent_mines {
            for i in (x.saturating_sub(1))..=(x + 1).min(minefield.width - 1) {
                for j in (y.saturating_sub(1))..=(y + 1).min(minefield.height - 1) {
                    if !(i == x && j == y) && !minefield.grid.contains(&(j, i)) {
                        reveal_adjacent_zeros(minefield, revealed, flagged, i, j);
                    }
                }
            }
        }
    }
}
