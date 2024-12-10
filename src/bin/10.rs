advent_of_code::solution!(10);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut heads = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                heads.push((x, y));
            }
        }
    }

    let mut score = 0;
    for head in heads {
        let mut visited = HashSet::new();
        let mut path = vec![];
        path.push(head);
        while !path.is_empty() {
            let node = path.pop()?;
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            let node_val = grid[node.1][node.0];
            if node_val == 9 {
                score += 1;
            } else {
                let deltas = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                for d in deltas {
                    let (nx, ny) = (node.0 as isize + d.0, node.1 as isize + d.1);
                    if nx >= 0
                        && nx < grid[0].len() as isize
                        && ny >= 0
                        && ny < grid.len() as isize
                        && grid[ny as usize][nx as usize] == node_val + 1
                    {
                        path.push((nx as usize, ny as usize));
                    }
                }
            }
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut heads = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                heads.push((x, y));
            }
        }
    }

    let mut score = 0;
    for head in heads {
        let mut path = vec![];
        path.push(head);
        while !path.is_empty() {
            let node = path.pop()?;
            let node_val = grid[node.1][node.0];
            if node_val == 9 {
                score += 1;
            } else {
                let deltas = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                for d in deltas {
                    let (nx, ny) = (node.0 as isize + d.0, node.1 as isize + d.1);
                    if nx >= 0
                        && nx < grid[0].len() as isize
                        && ny >= 0
                        && ny < grid.len() as isize
                        && grid[ny as usize][nx as usize] == node_val + 1
                    {
                        path.push((nx as usize, ny as usize));
                    }
                }
            }
        }
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
