advent_of_code::solution!(1);
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut split = line.split_whitespace();
            left.push(split.next().unwrap().parse::<i32>().unwrap());
            right.push(split.next().unwrap().parse::<i32>().unwrap());
        });

    Some(
        left.into_iter()
            .sorted()
            .zip(right.into_iter().sorted())
            .map(|(x, y)| (x - y).abs())
            .sum::<i32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut split = line.split_whitespace();
            left.push(split.next().unwrap().parse::<u32>().unwrap());
            let right_val = split.next().unwrap().parse::<u32>().unwrap();
            *right.entry(right_val).or_insert(0) += 1;
        });

    Some(
        left.into_iter()
            .map(|x| x * right.get(&x).unwrap_or(&0))
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
