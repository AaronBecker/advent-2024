advent_of_code::solution!(6);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let dirs = ['^', '>', 'v', '<'];
    let deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let (mut cx, mut cy, mut cd): (isize, isize, usize) = (0, 0, 0);
    'grid: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if let Some(idx) = dirs.into_iter().position(|e| e == grid[y][x]) {
                (cx, cy, cd) = (x as isize, y as isize, idx);
                break 'grid;
            }
        }
    }
    let mut positions = 0;
    loop {
        if grid[cy as usize][cx as usize] != 'X' {
            grid[cy as usize][cx as usize] = 'X';
            positions += 1;
        }
        let (nx, ny) = (cx + deltas[cd].0, cy + deltas[cd].1);
        if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
            break;
        }
        match grid[ny as usize][nx as usize] {
            '#' => {
                cd = (cd + 1) % 4;
            }
            _ => {
                (cx, cy) = (nx, ny);
            }
        }
    }
    Some(positions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let dirs = ['^', '>', 'v', '<'];
    let deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let (mut cx, mut cy, mut cd): (isize, isize, usize) = (0, 0, 0);
    'grid: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if let Some(idx) = dirs.into_iter().position(|e| e == grid[y][x]) {
                (cx, cy, cd) = (x as isize, y as isize, idx);
                break 'grid;
            }
        }
    }
    let (sx, sy, sd) = (cx, cy, cd);
    let mut candidates = HashSet::new();
    loop {
        if grid[cy as usize][cx as usize] != 'X' {
            grid[cy as usize][cx as usize] = 'X';
            if !(cx == sx && cy == sy) {
                candidates.insert((cx, cy));
            }
        }
        let (nx, ny) = (cx + deltas[cd].0, cy + deltas[cd].1);
        if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
            break;
        }
        match grid[ny as usize][nx as usize] {
            '#' => {
                cd = (cd + 1) % 4;
            }
            _ => {
                (cx, cy) = (nx, ny);
            }
        }
    }

    let mut blockers = 0;
    for (ox, oy) in candidates.iter() {
        // Reset state and place obstacle
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 'X' {
                    grid[y][x] = '.';
                }
            }
        }
        (cx, cy, cd) = (sx, sy, sd);
        grid[*oy as usize][*ox as usize] = '#';
        let mut seen_states = HashSet::new();
        loop {
            if grid[cy as usize][cx as usize] != 'X' {
                grid[cy as usize][cx as usize] = 'X';
            }
            if !seen_states.contains(&(cx, cy, cd)) {
                seen_states.insert((cx, cy, cd));
            } else {
                blockers += 1;
                grid[*oy as usize][*ox as usize] = '.';
                break;
            }
            let (nx, ny) = (cx + deltas[cd].0, cy + deltas[cd].1);
            if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
                grid[*oy as usize][*ox as usize] = '.';
                break;
            }
            match grid[ny as usize][nx as usize] {
                '#' => {
                    cd = (cd + 1) % 4;
                }
                _ => {
                    (cx, cy) = (nx, ny);
                }
            }
        }
    }
    Some(blockers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
