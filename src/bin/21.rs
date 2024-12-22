advent_of_code::solution!(21);
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;

fn key_to_position(k: char) -> (i32, i32) {
    match k {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("invalid key"),
    }
}
fn dir_to_position(k: char) -> (i32, i32) {
    match k {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("invalid key"),
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Pad {
    Keypad,
    Dpad,
}

fn legal_path(path: &Vec<char>, start: (i32, i32), pad: Pad) -> bool {
    let blank = if pad == Pad::Keypad { (0, 3) } else { (0, 0) };
    let mut pos = start;
    for p in path {
        let d = match p {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("invalid path"),
        };
        let npos = (pos.0 + d.0, pos.1 + d.1);
        if npos.0 == blank.0 && npos.1 == blank.1 {
            return false;
        }
        pos = (npos.0, npos.1);
    }
    true
}

fn basic_path(s: (i32, i32), e: (i32, i32)) -> Vec<char> {
    let mut seq = vec![];
    let hchar = if s.0 < e.0 { '>' } else { '<' };
    let vchar = if s.1 < e.1 { 'v' } else { '^' };
    for _ in 0..(e.0 - s.0).abs() {
        seq.push(hchar);
    }
    for _ in 0..(e.1 - s.1).abs() {
        seq.push(vchar);
    }
    seq
}

fn min_path_len(code: &str, iterations: usize) -> u64 {
    let mut options = vec![vec![vec!['A']]];
    for w in code.as_bytes().windows(2) {
        let (s, e) = (key_to_position(w[0] as char), key_to_position(w[1] as char));
        let bpath = basic_path(s, e);
        let blen = bpath.len();
        let mut segment_options = HashSet::new();
        for mut p in bpath.into_iter().permutations(blen) {
            if legal_path(&p, s, Pad::Keypad) {
                p.push('A');
                segment_options.insert(p);
            }
        }
        options.push(Vec::from_iter(segment_options));
    }

    let mut best_length = u64::max_value();
    for p in options.into_iter().multi_cartesian_product() {
        let path = p.into_iter().flatten().collect_vec();
        let command = &path.into_iter().join("");
        let length = expand_dpad_commands(&command, iterations);
        if length < best_length {
            best_length = length;
        }
    }

    best_length
}

fn expand_dpad_commands(commands: &str, iterations: usize) -> u64 {
    let mut pairs: HashMap<(char, char), u64> = HashMap::new();
    for w in commands.as_bytes().windows(2) {
        *pairs.entry((w[0] as char, w[1] as char)).or_insert(0) += 1;
    }
    for _ in 0..iterations {
        let mut next_pairs = HashMap::new();
        for (k, v) in pairs {
            match (k.0, k.1) {
                ('A', '^') => {
                    // A<A
                    *next_pairs.entry(('A', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', 'A')).or_insert(0) += v;
                }
                ('A', '>') => {
                    // AvA
                    *next_pairs.entry(('A', 'v')).or_insert(0) += v;
                    *next_pairs.entry(('v', 'A')).or_insert(0) += v;
                }
                ('A', 'v') => {
                    // A<vA
                    *next_pairs.entry(('A', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', 'v')).or_insert(0) += v;
                    *next_pairs.entry(('v', 'A')).or_insert(0) += v;
                }
                ('A', '<') => {
                    // Av<<A
                    *next_pairs.entry(('A', 'v')).or_insert(0) += v;
                    *next_pairs.entry(('v', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', 'A')).or_insert(0) += v;
                }
                ('A', 'A') => {
                    // AA
                    *next_pairs.entry(('A', 'A')).or_insert(0) += v;
                }

                ('^', '^') => {
                    // AA
                    *next_pairs.entry(('A', 'A')).or_insert(0) += v;
                }
                ('^', '>') => {
                    // Av>A
                    *next_pairs.entry(('A', 'v')).or_insert(0) += v;
                    *next_pairs.entry(('v', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', 'A')).or_insert(0) += v;
                }
                ('^', 'v') => {
                    // AvA
                    *next_pairs.entry(('A', 'v')).or_insert(0) += v;
                    *next_pairs.entry(('v', 'A')).or_insert(0) += v;
                }
                ('^', '<') => {
                    // Av<A
                    *next_pairs.entry(('A', 'v')).or_insert(0) += v;
                    *next_pairs.entry(('v', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', 'A')).or_insert(0) += v;
                }
                ('^', 'A') => {
                    // A>A
                    *next_pairs.entry(('A', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', 'A')).or_insert(0) += v;
                }

                ('>', '^') => {
                    // A<^A
                    *next_pairs.entry(('A', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', '^')).or_insert(0) += v;
                    *next_pairs.entry(('^', 'A')).or_insert(0) += v;
                }
                ('>', '>') => {
                    // AA
                    *next_pairs.entry(('A', 'A')).or_insert(0) += v;
                }
                ('>', 'v') => {
                    // A<A
                    *next_pairs.entry(('A', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', 'A')).or_insert(0) += v;
                }
                ('>', '<') => {
                    // A<<A
                    *next_pairs.entry(('A', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', 'A')).or_insert(0) += v;
                }
                ('>', 'A') => {
                    // A^A
                    *next_pairs.entry(('A', '^')).or_insert(0) += v;
                    *next_pairs.entry(('^', 'A')).or_insert(0) += v;
                }

                ('v', '^') => {
                    // A^A
                    *next_pairs.entry(('A', '^')).or_insert(0) += v;
                    *next_pairs.entry(('^', 'A')).or_insert(0) += v;
                }
                ('v', '>') => {
                    // A>A
                    *next_pairs.entry(('A', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', 'A')).or_insert(0) += v;
                }
                ('v', 'v') => {
                    // AA
                    *next_pairs.entry(('A', 'A')).or_insert(0) += v;
                }
                ('v', '<') => {
                    // A<A
                    *next_pairs.entry(('A', '<')).or_insert(0) += v;
                    *next_pairs.entry(('<', 'A')).or_insert(0) += v;
                }
                ('v', 'A') => {
                    // !!!
                    // A^>A
                    *next_pairs.entry(('A', '^')).or_insert(0) += v;
                    *next_pairs.entry(('^', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', 'A')).or_insert(0) += v;
                }

                ('<', '^') => {
                    // A>^A
                    *next_pairs.entry(('A', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', '^')).or_insert(0) += v;
                    *next_pairs.entry(('^', 'A')).or_insert(0) += v;
                }
                ('<', '>') => {
                    // A>>A
                    *next_pairs.entry(('A', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', 'A')).or_insert(0) += v;
                }
                ('<', 'v') => {
                    // A>A
                    *next_pairs.entry(('A', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', 'A')).or_insert(0) += v;
                }
                ('<', '<') => {
                    // AA
                    *next_pairs.entry(('A', 'A')).or_insert(0) += v;
                }
                ('<', 'A') => {
                    // A>>^A
                    *next_pairs.entry(('A', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', '>')).or_insert(0) += v;
                    *next_pairs.entry(('>', '^')).or_insert(0) += v;
                    *next_pairs.entry(('^', 'A')).or_insert(0) += v;
                }
                _ => panic!("bad pair ({}, {})", k.0, k.1),
            }
        }
        pairs = next_pairs;
    }

    pairs.values().sum::<u64>()
}

fn eval_input(input: &str, iterations: usize) -> Option<u64> {
    let mut total_cost = 0;
    for case in input.lines() {
        let command_len = min_path_len(&case, iterations);
        let cost = command_len
            * case
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .trim_start_matches('0')
                .parse::<u64>()
                .unwrap();
        total_cost += cost;
    }
    Some(total_cost)
}

pub fn part_one(input: &str) -> Option<u64> {
    eval_input(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    eval_input(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
