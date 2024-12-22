advent_of_code::solution!(22);
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn next(secret: u64) -> u64 {
    let s2 = secret ^ (secret * 64) % 16777216;
    let s3 = s2 ^ (s2 / 32) % 16777216;
    let s4 = s3 ^ (s3 * 2048) % 16777216;
    s4
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for mut secret in input.lines().map(|x| x.parse::<u64>().unwrap()) {
        for _i in 0..2000 {
            secret = next(secret);
        }
        total += secret;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut patterns = vec![];
    let mut candidates = HashSet::new();
    for (idx, mut secret) in input.lines().map(|x| x.parse::<u64>().unwrap()).enumerate() {
        patterns.push(HashMap::new());
        let mut history = VecDeque::new();
        for _i in 0..2000 {
            let next = next(secret);
            history.push_back((next % 10) as i32 - (secret % 10) as i32);
            if history.len() > 4 {
                history.pop_front();
            }
            if history.len() == 4 && !patterns[idx].contains_key(&history) {
                patterns[idx].insert(history.clone(), next % 10);
                candidates.insert(history.clone());
            }

            secret = next;
        }
    }

    candidates
        .iter()
        .map(|c| patterns.iter().map(|p| p.get(&c).unwrap_or(&0)).sum())
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "1\n\
             2\n\
             3\n\
             2024",
        );
        assert_eq!(result, Some(23));
    }
}
