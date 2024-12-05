advent_of_code::solution!(4);

pub fn matches(text: &[u8], target: &[u8], i: isize, delta: isize) -> bool {
    if (text.len() as isize) <= i + delta * 3 {
        return false;
    }
    for offset in 0..4 {
        if text[(i + offset * delta) as usize] != target[offset as usize] {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = (input.find('\n').unwrap() + 1) as isize;
    let mut found = 0;
    for i in 0..input.len() as isize {
        for delta in [1, width, width + 1, width - 1] {
            found += matches(&input.as_bytes(), "XMAS".as_bytes(), i, delta) as u32
                + matches(&input.as_bytes(), "SAMX".as_bytes(), i, delta) as u32
        }
    }
    Some(found)
}

pub fn crossmas(m1: char, m2: char, a: char, s1: char, s2: char) -> bool {
    a == 'A'
        && (m1 == 'M' && s2 == 'S' || m1 == 'S' && s2 == 'M')
        && (m2 == 'M' && s1 == 'S' || m2 == 'S' && s1 == 'M')
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
