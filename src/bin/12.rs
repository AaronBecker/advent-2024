advent_of_code::solution!(12);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut cost = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let mut region = vec![(x, y)];
            let (mut area, mut perimeter, symbol) = (0, 0, grid[y][x]);
            while !region.is_empty() {
                let (rx, ry) = region.pop()?;
                if visited.contains(&(rx, ry)) {
                    continue;
                }
                visited.insert((rx, ry));
                area += 1;
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (rx as isize + dx, ry as isize + dy);
                    if nx < 0
                        || nx == grid[0].len() as isize
                        || ny < 0
                        || ny == grid.len() as isize
                        || grid[ny as usize][nx as usize] != symbol
                    {
                        perimeter += 1;
                        continue;
                    }
                    if !visited.contains(&(nx as usize, ny as usize)) {
                        region.push((nx as usize, ny as usize));
                    }
                }
            }
            cost += area * perimeter;
        }
    }
    Some(cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut cost = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let mut region = vec![(x, y)];
            let mut perimeter = vec![];
            let (mut area, symbol) = (0, grid[y][x]);
            while !region.is_empty() {
                let (rx, ry) = region.pop()?;
                if visited.contains(&(rx, ry)) {
                    continue;
                }
                visited.insert((rx, ry));
                area += 1;
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (rx as isize + dx, ry as isize + dy);
                    if nx < 0
                        || nx == grid[0].len() as isize
                        || ny < 0
                        || ny == grid.len() as isize
                        || grid[ny as usize][nx as usize] != symbol
                    {
                        perimeter.push(((nx, ny), (dx, dy)));
                        continue;
                    }
                    if !visited.contains(&(nx as usize, ny as usize)) {
                        region.push((nx as usize, ny as usize));
                    }
                }
            }
            let mut sides = 0;
            for ((cx, cy), delta) in &perimeter {
                let (nx, ny) = match delta {
                    (-1, 0) => (*cx, cy + 1),
                    (1, 0) => (*cx, cy + 1),
                    (0, -1) => (cx + 1, *cy),
                    (0, 1) => (cx + 1, *cy),
                    _ => panic!("bad delta"),
                };
                if !perimeter.contains(&((nx, ny), *delta)) {
                    sides += 1;
                }
            }
            cost += area * sides;
        }
    }
    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(140));
        let result = part_one("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO");
        assert_eq!(result, Some(772));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(80));
        let result = part_two("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO");
        assert_eq!(result, Some(436));
        let result = part_two("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE");
        assert_eq!(result, Some(236));
        let result = part_two("AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA");
        assert_eq!(result, Some(368));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
