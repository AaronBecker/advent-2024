advent_of_code::solution!(18);
use sscanf::sscanf;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u64> {
    let mut bytes = vec![];
    for line in input.lines() {
        bytes.push(sscanf!(line, "{i32},{i32}").unwrap());
    }
    assert!(bytes.len() >= 1024);

    let mut fallen = HashSet::new();
    let mut visited = HashSet::new();
    for i in 0..1024 {
        fallen.insert(bytes[i]);
    }
    let mut next = VecDeque::from([((0, 0), 0)]);
    while !next.is_empty() {
        if let Some((pos, steps)) = next.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            if pos == (70, 70) {
                return Some(steps);
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                if 0 <= dx + pos.0
                    && 70 >= dx + pos.0
                    && 0 <= dy + pos.1
                    && 70 >= dy + pos.1
                    && !fallen.contains(&pos)
                {
                    next.push_back(((pos.0 + dx, pos.1 + dy), steps + 1));
                }
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    let mut bytes = vec![];
    for line in input.lines() {
        bytes.push(sscanf!(line, "{i32},{i32}").unwrap());
    }

    let mut fallen = HashSet::new();
    for time in 0..bytes.len() {
        fallen.insert(bytes[time]);
        let mut visited = HashSet::new();
        let mut next = VecDeque::from([((0, 0), 0)]);
        let mut found = false;
        while !next.is_empty() {
            if let Some((pos, steps)) = next.pop_front() {
                if visited.contains(&pos) {
                    continue;
                }
                visited.insert(pos);
                if pos == (70, 70) {
                    found = true;
                    break;
                }
                for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                    if 0 <= dx + pos.0
                        && 70 >= dx + pos.0
                        && 0 <= dy + pos.1
                        && 70 >= dy + pos.1
                        && !fallen.contains(&pos)
                    {
                        next.push_back(((pos.0 + dx, pos.1 + dy), steps + 1));
                    }
                }
            }
        }
        if !found {
            return Some(format!("{},{}", bytes[time].0, bytes[time].1));
        }
    }
    None
}
