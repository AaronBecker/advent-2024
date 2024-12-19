advent_of_code::solution!(19);
use std::collections::HashMap;

fn build_target(memo: &mut HashMap<String, u64>, pieces: &Vec<&str>, target: &str) -> u64 {
    if target.is_empty() {
        return 1;
    }
    if memo.contains_key(target) {
        return *memo.get(target).unwrap();
    }
    let mut ways = 0;
    for piece in pieces {
        if let Some(remain) = target.strip_prefix(piece) {
            ways += build_target(memo, pieces, remain);
        }
    }
    memo.insert(target.to_string(), ways);
    ways
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let towels: Vec<_> = lines.next().unwrap().split(", ").collect();
    let _ = lines.next();
    let targets: Vec<_> = lines.collect();

    let mut memo = HashMap::new();
    Some(
        targets
            .into_iter()
            .filter(|t| build_target(&mut memo, &towels, t) > 0)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let towels: Vec<_> = lines.next().unwrap().split(", ").collect();
    let _ = lines.next();
    let targets: Vec<_> = lines.collect();

    let mut memo = HashMap::new();
    Some(
        targets
            .into_iter()
            .map(|t| build_target(&mut memo, &towels, t))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
