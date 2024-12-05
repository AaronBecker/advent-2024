advent_of_code::solution!(5);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lines = input.lines().into_iter();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (x, y) = sscanf::sscanf!(line, "{u32}|{u32}").unwrap();
        before.entry(y).or_insert(vec![]).push(x);
    }
    let mut mid_sum = 0;
    'updates: for line in lines {
        if line.is_empty() {
            continue;
        }
        let update: Vec<_> = line.split(',').map(|u| u.parse::<u32>().unwrap()).collect();
        for i in 0..update.len() {
            match before.get(&update[i]) {
                Some(before_i) => {
                    for j in i..update.len() {
                        if before_i.contains(&update[j]) {
                            continue 'updates;
                        }
                    }
                }
                None => continue,
            }
        }
        mid_sum += update[update.len() / 2]
    }
    Some(mid_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lines = input.lines().into_iter();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (x, y) = sscanf::sscanf!(line, "{u32}|{u32}").unwrap();
        before.entry(y).or_insert(vec![]).push(x);
    }
    let mut mid_sum = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut update: Vec<_> = line.split(',').map(|u| u.parse::<u32>().unwrap()).collect();
        let mut corrected = false;
        for i in 0..update.len() {
            'fix_constraints: loop {
                match before.get(&update[i]) {
                    Some(before_i) => {
                        for j in i..update.len() {
                            if before_i.contains(&update[j]) {
                                corrected = true;
                                update.swap(i, j);
                                continue 'fix_constraints;
                            }
                        }
                        break;
                    }
                    None => break,
                }
            }
        }
        if corrected {
            mid_sum += update[update.len() / 2]
        }
    }
    Some(mid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
