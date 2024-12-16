advent_of_code::solution!(16);
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    cost: u64,
    pos: (isize, isize),
    dir: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            if let Some(x) = l.find('S') {
                start = (x as isize, idx as isize)
            }
            if let Some(x) = l.find('E') {
                end = (x as isize, idx as isize)
            }
            l.chars().collect::<Vec<_>>()
        })
        .collect();

    let deltas = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut next = BinaryHeap::new();
    let mut min_cost = HashMap::new();
    next.push(Reverse(State {
        pos: start,
        dir: 0,
        cost: 0,
    }));
    min_cost.insert((start, 0), 0);
    while !next.is_empty() {
        if let Some(Reverse(node)) = next.pop() {
            if node.pos == end {
                return Some(node.cost);
            }
            let forward = (
                node.pos.0 as isize + deltas[node.dir].0,
                node.pos.1 as isize + deltas[node.dir].1,
            );
            if 0 <= forward.0
                && forward.0 < grid[0].len() as isize
                && 0 <= forward.1
                && forward.1 < grid.len() as isize
                && grid[forward.1 as usize][forward.0 as usize] != '#'
            {
                let val = min_cost.entry((forward, node.dir)).or_insert(u64::MAX);
                if node.cost + 1 < *val {
                    *val = node.cost + 1;
                    next.push(Reverse(State {
                        pos: forward,
                        dir: node.dir,
                        cost: *val,
                    }));
                }
            }
            let val = min_cost
                .entry((node.pos, (node.dir + 1) % 4))
                .or_insert(u64::MAX);
            if node.cost + 1000 < *val {
                *val = node.cost + 1000;
                next.push(Reverse(State {
                    pos: node.pos,
                    dir: (node.dir + 1) % 4,
                    cost: *val,
                }));
            }
            let val = min_cost
                .entry((node.pos, (node.dir + 3) % 4))
                .or_insert(u64::MAX);
            if node.cost + 1000 < *val {
                *val = node.cost + 1000;
                next.push(Reverse(State {
                    pos: node.pos,
                    dir: (node.dir + 3) % 4,
                    cost: *val,
                }));
            }
        } else {
            panic!("invalid node");
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            if let Some(x) = l.find('S') {
                start = (x as isize, idx as isize)
            }
            if let Some(x) = l.find('E') {
                end = (x as isize, idx as isize)
            }
            l.chars().collect::<Vec<_>>()
        })
        .collect();

    let deltas = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut next = BinaryHeap::new();
    let mut prev = HashMap::new();
    let mut min_cost = HashMap::new();
    next.push(Reverse(State {
        pos: start,
        dir: 0,
        cost: 0,
    }));
    min_cost.insert((start, 0), 0);
    while !next.is_empty() {
        if let Some(Reverse(node)) = next.pop() {
            if node.pos == end {
                break;
            }

            let mut next_states = vec![
                State {
                    pos: node.pos,
                    dir: (node.dir + 1) % 4,
                    cost: node.cost + 1000,
                },
                State {
                    pos: node.pos,
                    dir: (node.dir + 3) % 4,
                    cost: node.cost + 1000,
                },
            ];
            let forward = (
                node.pos.0 as isize + deltas[node.dir].0,
                node.pos.1 as isize + deltas[node.dir].1,
            );
            if 0 <= forward.0
                && forward.0 < grid[0].len() as isize
                && 0 <= forward.1
                && forward.1 < grid.len() as isize
                && grid[forward.1 as usize][forward.0 as usize] != '#'
            {
                next_states.push(State {
                    pos: forward,
                    dir: node.dir,
                    cost: node.cost + 1,
                });
            }
            for state in next_states {
                let val = min_cost.entry((state.pos, state.dir)).or_insert(u64::MAX);
                if state.cost <= *val {
                    if state.cost < *val {
                        *val = state.cost;
                        prev.insert((state.pos, state.dir), vec![(node.pos, node.dir)]);
                        next.push(Reverse(state));
                    } else {
                        prev.entry((state.pos, state.dir))
                            .or_insert(vec![])
                            .push((node.pos, node.dir));
                    }
                }
            }
        }
    }

    let mut prev_stack = vec![];
    for ((pos, _dir), prev_states) in &prev {
        if *pos == end {
            prev_stack.extend(prev_states);
        }
    }
    let mut shortest_path_positions = HashSet::new();
    shortest_path_positions.insert(end);
    while !prev_stack.is_empty() {
        let (pos, dir) = prev_stack.pop().unwrap();
        shortest_path_positions.insert(pos);
        if let Some(prev_nodes) = prev.get(&(pos, dir)) {
            prev_stack.extend(prev_nodes);
        }
    }

    Some(shortest_path_positions.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
