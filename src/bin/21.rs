advent_of_code::solution!(21);
use itertools::Itertools;
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

fn min_path(start: char, end: char, iterations: usize) -> Vec<char> {
    let (s, e) = (key_to_position(start), key_to_position(end));
    let seq = basic_path(s, e);
    let mut paths = HashSet::new();
    let slen = seq.len();
    for p in seq.into_iter().permutations(slen) {
        paths.insert(p);
    }
    //println!("paths from {} to {}", start, end);
    let mut cur_paths = vec![];
    for p in paths {
        if legal_path(&p, s, Pad::Keypad) {
            let mut full_path = vec!['A'];
            full_path.extend(p);
            full_path.push('A');
            cur_paths.push(full_path);
        }
    }
    //println!("d1");
    //for p in &d1_paths {
    //    println!("{}", p.into_iter().join(""));
    //}
    for i in 0..iterations {
        println!("iteration {}", i);
        let mut next_paths: Vec<Vec<char>> = vec![];
        for p in &cur_paths {
            println!("expanding path with length {}", p.len());
            let mut next_options = vec![vec![vec!['A']]];
            for w in p.windows(2) {
                let (s, e) = (dir_to_position(w[0]), dir_to_position(w[1]));
                let bpath = basic_path(s, e);
                let blen = bpath.len();
                let mut options = vec![];
                for p in bpath.into_iter().permutations(blen) {
                    if legal_path(&p, s, Pad::Dpad) {
                        options.push(p);
                        options.last_mut().unwrap().push('A');
                    }
                }
                next_options.push(options);
            }
            let mut best_length = usize::max_value();
            for p in next_options.into_iter().multi_cartesian_product() {
                let path = p.into_iter().flatten().collect_vec();
                if path.len() <= best_length {
                    best_length = path.len();
                    next_paths.push(path);
                }
            }
        }
        mem::swap(&mut cur_paths, &mut next_paths);
        next_paths.clear();
    }
    //println!("out");
    //for p in &cur_paths {
    //    println!("{}", p.into_iter().join(""));
    //}
    cur_paths.into_iter().min_by_key(|p| p.len()).unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_cost = 0;
    for case in input.lines() {
        println!("case={}", case);
        let case = "A".to_owned() + case;
        let mut commands = String::new();
        for w in case.as_bytes().windows(2) {
            commands += &min_path(w[0] as char, w[1] as char, 2).into_iter().join("")[1..];
            println!("partial commands: {}", commands);
        }
        let cost = commands.len()
            * case
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .trim_start_matches('0')
                .parse::<usize>()
                .unwrap();
        total_cost += cost;

        println!(
            "{} * {}",
            commands.len(),
            case.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .trim_start_matches('0')
                .parse::<usize>()
                .unwrap()
        );
        println!("{}", cost);
    }
    Some(total_cost as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_cost = 0;
    for case in input.lines() {
        println!("case={}", case);
        let case = "A".to_owned() + case;
        let mut commands = String::new();
        for w in case.as_bytes().windows(2) {
            commands += &min_path(w[0] as char, w[1] as char, 3).into_iter().join("")[1..];
            println!("partial commands: {}", commands);
        }
        let cost = commands.len()
            * case
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .trim_start_matches('0')
                .parse::<usize>()
                .unwrap();
        total_cost += cost;

        println!(
            "{} * {}",
            commands.len(),
            case.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .trim_start_matches('0')
                .parse::<usize>()
                .unwrap()
        );
        println!("{}", cost);
    }
    Some(total_cost as u64)
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
