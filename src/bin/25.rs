advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u64> {
    let parts = input.split("\n\n");
    let mut keys: Vec<_> = vec![];
    let mut locks: Vec<_> = vec![];
    for part in parts {
        let is_lock = part.chars().nth(0) == Some('#');
        let mut heights = vec![-1, -1, -1, -1, -1];

        for line in part.lines() {
            for (i, c) in line.chars().enumerate() {
                heights[i] += (c == '#') as i32
            }
        }
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut matches = 0;
    for key in keys {
        for lock in &locks {
            for i in 0..5 {
                if key[i] + lock[i] >= 6 {
                    break;
                }
                if i == 4 {
                    matches += 1;
                }
            }
        }
    }
    Some(matches)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
