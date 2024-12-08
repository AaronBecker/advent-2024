advent_of_code::solution!(7);
use itertools::Itertools;
use std::iter;

pub fn part_one(input: &str) -> Option<u64> {
    let mut target_sum = 0;
    for case in input.lines() {
        let (target, rest) = case.split_once(':')?;
        let (target, vals) = (
            target.parse::<u64>().unwrap(),
            rest.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec(),
        );
        for ops in iter::repeat(['+', '*'].iter())
            .take(vals.len() - 1)
            .multi_cartesian_product()
        {
            let mut res = vals[0];
            for i in 0..vals.len() - 1 {
                res = match ops[i] {
                    '+' => res + vals[i + 1],
                    '*' => res * vals[i + 1],
                    _ => panic!(),
                }
            }
            if res == target {
                target_sum += target;
                break;
            }
        }
    }
    Some(target_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut target_sum = 0;
    for case in input.lines() {
        let (target, rest) = case.split_once(':')?;
        let (target, vals) = (
            target.parse::<u64>().unwrap(),
            rest.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec(),
        );
        for ops in iter::repeat(['+', '*', '|'].iter())
            .take(vals.len() - 1)
            .multi_cartesian_product()
        {
            let mut res = vals[0];
            for i in 0..vals.len() - 1 {
                res = match ops[i] {
                    '+' => res + vals[i + 1],
                    '*' => res * vals[i + 1],
                    '|' => res * 10u64.pow(vals[i + 1].ilog10() + 1) + vals[i + 1],
                    _ => panic!(),
                }
            }
            if res == target {
                target_sum += target;
                break;
            }
        }
    }
    Some(target_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
