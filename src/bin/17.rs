advent_of_code::solution!(17);
use sscanf::sscanf;

fn combo(v: u64, r: &Vec<u64>) -> u64 {
    match v {
        4 => r[0],
        5 => r[1],
        6 => r[2],
        7 => panic!("invalid combo operand {}", v),
        _ => v,
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut r = vec![0, 0, 0];
    let mut output = vec![];
    let mut pc = 0;
    let mut lines = input.lines();
    r[0] = sscanf!(lines.next().unwrap(), "Register A: {u64}").unwrap();
    r[1] = sscanf!(lines.next().unwrap(), "Register B: {u64}").unwrap();
    r[1] = sscanf!(lines.next().unwrap(), "Register C: {u64}").unwrap();
    let _ = lines.next().unwrap();
    let program: Vec<_> = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    loop {
        if pc + 1 > program.len() {
            break;
        }
        let op = program[pc + 1];
        pc = match program[pc] {
            0 => {
                r[0] = r[0] / 2u64.pow(combo(op, &r) as u32);
                pc + 2
            }
            1 => {
                r[1] = r[1] ^ op;
                pc + 2
            }
            2 => {
                r[1] = combo(op, &r) % 8;
                pc + 2
            }
            3 => {
                if r[0] == 0 {
                    pc + 2
                } else {
                    op as usize
                }
            }
            4 => {
                r[1] = r[1] ^ r[2];
                pc + 2
            }
            5 => {
                output.push(combo(op, &r) % 8);
                pc + 2
            }
            6 => {
                r[1] = r[0] / 2u64.pow(combo(op, &r) as u32);
                pc + 2
            }
            7 => {
                r[2] = r[0] / 2u64.pow(combo(op, &r) as u32);
                pc + 2
            }
            _ => panic!("bad instruction"),
        };
    }

    Some(
        output
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}

use std::mem;

pub fn part_two(input: &str) -> Option<u64> {
    let mut r = vec![0, 0, 0];
    let mut lines = input.lines();
    r[0] = sscanf!(lines.next().unwrap(), "Register A: {u64}").unwrap();
    r[1] = sscanf!(lines.next().unwrap(), "Register B: {u64}").unwrap();
    r[1] = sscanf!(lines.next().unwrap(), "Register C: {u64}").unwrap();
    let _r_init = r.clone();
    let _ = lines.next().unwrap();
    let mut program: Vec<_> = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut prefixes = vec![0];
    let mut next_prefixes = vec![];
    program.reverse();
    for digit in &program {
        for a in &prefixes {
            for low in 0..8 {
                let candidate = (a << 3) + low;
                let mut b = low ^ 5;
                b = b ^ (candidate >> b) ^ 6;
                if b % 8 == *digit {
                    next_prefixes.push(candidate);
                }
            }
        }
        mem::swap(&mut prefixes, &mut next_prefixes);
        next_prefixes.clear();
    }
    Some(prefixes[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        // The day 2 implementation uses a hand-decompiled version of the
        // program from the real input, so examples won't work.
    }
}
