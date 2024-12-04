advent_of_code::solution!(4);

pub fn xmas(x: char, m: char, a: char, s: char) -> bool {
    (x == 'X' && m == 'M' && a == 'A' && s == 'S') || (x == 'S' && m == 'A' && a == 'M' && s == 'X')
}

pub fn crossmas(m1: char, m2: char, a: char, s1: char, s2: char) -> bool {
    a == 'A'
        && (m1 == 'M' && s2 == 'S' || m1 == 'S' && s2 == 'M')
        && (m2 == 'M' && s1 == 'S' || m2 == 'S' && s1 == 'M')
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut found = input.matches("XMAS").count() + input.matches("SAMX").count();
    let grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    for y in 0..grid.len() - 3 {
        for x in 0..grid[0].len() {
            // vertical
            if xmas(grid[y][x], grid[y + 1][x], grid[y + 2][x], grid[y + 3][x]) {
                found += 1;
            }
            // right diagonal
            if x < grid[0].len() - 3
                && xmas(
                    grid[y][x],
                    grid[y + 1][x + 1],
                    grid[y + 2][x + 2],
                    grid[y + 3][x + 3],
                )
            {
                found += 1;
            }
            // left diagonal
            if x < grid[0].len() - 3
                && xmas(
                    grid[y][x + 3],
                    grid[y + 1][x + 2],
                    grid[y + 2][x + 1],
                    grid[y + 3][x],
                )
            {
                found += 1;
            }
        }
    }
    Some(found)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut found = 0;
    let grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    for y in 0..grid.len() - 2 {
        for x in 0..grid[0].len() - 2 {
            if crossmas(
                grid[y][x],
                grid[y + 2][x],
                grid[y + 1][x + 1],
                grid[y][x + 2],
                grid[y + 2][x + 2],
            ) {
                found += 1;
            }
        }
    }
    Some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
