advent_of_code::solution!(11);
use std::collections::HashMap;
use std::mem;

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut new_stones = Vec::new();
    for _iter in 0..25 {
        for s in &stones {
            if *s == 0 {
                new_stones.push(1);
            } else if s.ilog10() % 2 == 1 {
                let mid = 10u64.pow((s.ilog10() + 1) / 2);
                new_stones.push(s / mid);
                new_stones.push(s % mid);
            } else {
                new_stones.push(s * 2024);
            }
        }
        mem::swap(&mut stones, &mut new_stones);
        new_stones.clear();
    }

    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    input.trim().split_whitespace().for_each(|n| {
        *stones.entry(n.parse::<u64>().unwrap()).or_insert(0) += 1;
    });

    let mut new_stones = HashMap::new();
    for _iter in 0..75 {
        for (s, count) in &stones {
            if *s == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if s.ilog10() % 2 == 1 {
                let mid = 10u64.pow((s.ilog10() + 1) / 2);
                *new_stones.entry(s / mid).or_insert(0) += count;
                *new_stones.entry(s % mid).or_insert(0) += count;
            } else {
                *new_stones.entry(s * 2024).or_insert(0) += count;
            }
        }
        mem::swap(&mut stones, &mut new_stones);
        new_stones.clear();
    }

    Some(stones.values().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
