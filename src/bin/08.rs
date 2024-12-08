advent_of_code::solution!(8);
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let mut stations = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '.' {
                stations
                    .entry(grid[y][x])
                    .or_insert(vec![])
                    .push((x as isize, y as isize));
            }
        }
    }
    let mut antinodes = 0;
    for positions in stations.values() {
        for pair in positions.iter().permutations(2) {
            if let [p1, p2] = pair.as_slice() {
                let (dx, dy) = (p2.0 - p1.0, p2.1 - p1.1);
                let (ax, ay) = (p2.0 + dx, p2.1 + dy);
                if ax < 0
                    || ax >= grid[0].len() as isize
                    || ay < 0
                    || ay >= grid.len() as isize
                    || grid[ay as usize][ax as usize] == '#'
                {
                    continue;
                }
                antinodes += 1;
                grid[ay as usize][ax as usize] = '#';
            }
        }
    }
    Some(antinodes)
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let mut stations = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '.' {
                stations
                    .entry(grid[y][x])
                    .or_insert(vec![])
                    .push((x as isize, y as isize));
            }
        }
    }
    let mut antinodes = 0;
    for positions in stations.values() {
        for pair in positions.iter().permutations(2) {
            if let [p1, p2] = pair.as_slice() {
                let (dx, dy) = (p2.0 - p1.0, p2.1 - p1.1);
                let (dx2, dy2) = (dx / gcd(dx, dy), dy / gcd(dx, dy));
                let (mut cx, mut cy) = (p2.0, p2.1);
                loop {
                    if grid[cy as usize][cx as usize] != '#' {
                        antinodes += 1;
                        grid[cy as usize][cx as usize] = '#';
                    }
                    (cx, cy) = (cx + dx2, cy + dy2);
                    if cx < 0 || cx >= grid[0].len() as isize || cy < 0 || cy >= grid.len() as isize
                    {
                        break;
                    }
                }
            }
        }
    }
    Some(antinodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
