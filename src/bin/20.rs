advent_of_code::solution!(20);
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn distances(grid: &Vec<Vec<char>>, start: (isize, isize)) -> HashMap<(usize, usize), usize> {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut next = VecDeque::new();
    next.push_back((start, 0));
    while !next.is_empty() {
        let (pos, dist) = next.pop_front().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        distances.insert((pos.0 as usize, pos.1 as usize), dist);
        for d in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
            if pos.0 + d.0 >= 0
                && pos.0 + d.0 < grid[0].len() as isize
                && pos.1 + d.1 >= 0
                && pos.1 + d.1 < grid.len() as isize
                && grid[pos.1 as usize][pos.0 as usize] != '#'
            {
                let npos = (pos.0 as isize + d.0, pos.1 as isize + d.1);
                next.push_back((npos, dist + 1));
            }
        }
    }
    distances
}

fn count_cheats(
    grid: &Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),
    duration: isize,
    min_improvement: usize,
) -> u64 {
    let start_dist = distances(&grid, start);
    let end_dist = distances(&grid, end);
    let base_time = *start_dist.get(&(end.0 as usize, end.1 as usize)).unwrap();
    let mut good_cheats = 0;

    for x1 in 0..grid[0].len() {
        for x2 in 0..grid[0].len() {
            for y1 in 0..grid.len() {
                for y2 in 0..grid.len() {
                    if grid[y1][x1] == '#' || grid[y2][x2] == '#' {
                        continue;
                    }
                    let cheat_dist =
                        (y1 as isize - y2 as isize).abs() + (x1 as isize - x2 as isize).abs();
                    if cheat_dist > duration {
                        continue;
                    }
                    let time = start_dist.get(&(x1, y1)).unwrap()
                        + cheat_dist as usize
                        + end_dist.get(&(x2, y2)).unwrap();
                    if time + min_improvement <= base_time {
                        good_cheats += 1;
                    }
                }
            }
        }
    }
    good_cheats
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (isize, isize), (isize, isize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            if let Some(x) = l.find('S') {
                start = (x as isize, idx as isize)
            }
            if let Some(x) = l.find('E') {
                end = (x as isize, idx as isize)
            }
            l.chars().collect::<Vec<_>>()
        })
        .collect();
    (grid, start, end)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start, end) = parse_input(input);
    Some(count_cheats(&grid, start, end, 2, 100))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start, end) = parse_input(input);
    Some(count_cheats(&grid, start, end, 20, 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
