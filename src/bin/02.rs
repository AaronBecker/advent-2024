advent_of_code::solution!(2);
use itertools::Itertools;

fn safe(levels: &[i32]) -> bool {
    let (mut inc, mut dec) = (false, false);
    for w in levels.windows(2) {
        let diff = (w[0] - w[1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        if w[0] < w[1] {
            inc = true
        } else if w[0] > w[1] {
            dec = true
        }
    }
    !(inc && dec)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    let lines = input.split('\n').filter(|line| !line.is_empty());
    for line in lines {
        let levels: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if safe(&levels) {
            safe_reports += 1
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    let lines = input.split('\n').filter(|line| !line.is_empty());
    for line in lines {
        let levels: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        for l2 in levels.clone().into_iter().combinations(levels.len() - 1) {
            if safe(&l2) {
                safe_reports += 1;
                break;
            }
        }
    }
    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
