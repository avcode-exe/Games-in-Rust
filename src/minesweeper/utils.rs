use rand::Rng;

pub fn generate_minefield(width: usize, height: usize, mines: usize) -> Vec<Vec<bool>> {
    let mut minefield = vec![vec![false; width]; height];
    let mut rng = rand::thread_rng();
    let mut placed_mines = 0;

    while placed_mines < mines {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        if !minefield[y][x] {
            minefield[y][x] = true;
            placed_mines += 1;
        }
    }

    minefield
}

pub fn count_adjacent_mines(minefield: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for i in (x.saturating_sub(1))..=(x + 1).min(minefield[0].len() - 1) {
        for j in (y.saturating_sub(1))..=(y + 1).min(minefield.len() - 1) {
            if minefield[j][i] {
                count += 1;
            }
        }
    }
    count
}

pub fn reveal_adjacent_zeros(
    minefield: &Vec<Vec<bool>>,
    revealed: &mut Vec<Vec<bool>>,
    flagged: &Vec<Vec<bool>>,
    x: usize,
    y: usize,
) {
    if revealed[y][x] {
        return;
    }
    revealed[y][x] = true;
    let adjacent_mines = count_adjacent_mines(minefield, x, y);
    if adjacent_mines == 0 {
        for i in (x.saturating_sub(1))..=(x + 1).min(minefield[0].len() - 1) {
            for j in (y.saturating_sub(1))..=(y + 1).min(minefield.len() - 1) {
                if !(i == x && j == y) {
                    reveal_adjacent_zeros(minefield, revealed, flagged, i, j);
                }
            }
        }
    } else {
        let mut flagged_count = 0;
        for i in (x.saturating_sub(1))..=(x + 1).min(minefield[0].len() - 1) {
            for j in (y.saturating_sub(1))..=(y + 1).min(minefield.len() - 1) {
                if flagged[j][i] {
                    flagged_count += 1;
                }
            }
        }
        if flagged_count == adjacent_mines {
            for i in (x.saturating_sub(1))..=(x + 1).min(minefield[0].len() - 1) {
                for j in (y.saturating_sub(1))..=(y + 1).min(minefield.len() - 1) {
                    if !(i == x && j == y) && !minefield[j][i] {
                        reveal_adjacent_zeros(minefield, revealed, flagged, i, j);
                    }
                }
            }
        }
    }
}
