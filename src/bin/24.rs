advent_of_code::solution!(24);
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    op: &'a str,
    out: &'a str,
}

fn parse_input(input: &str) -> (HashMap<&str, Option<bool>>, Vec<Gate>) {
    let mut lines = input.lines();
    let mut registers = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<u8>().unwrap() != 0;
        registers.insert(name, Some(value));
    }
    let mut gates = vec![];
    while let Some(line) = lines.next() {
        let parts: Vec<_> = line.split(" ").collect();
        gates.push(Gate {
            a: parts[0],
            b: parts[2],
            op: parts[1],
            out: parts[4],
        });
        if !registers.contains_key(parts[4]) {
            registers.insert(parts[4], None);
        }
    }
    (registers, gates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut registers, gates) = parse_input(input);
    let mut updated = 1;
    while updated > 0 {
        updated = 0;
        for gate in &gates {
            let out = registers.get(gate.out).unwrap();
            if *out != None {
                continue;
            }
            if let Some(a) = *registers.get(gate.a).unwrap() {
                if let Some(b) = *registers.get(gate.b).unwrap() {
                    registers.insert(
                        gate.out,
                        Some(match gate.op {
                            "AND" => a && b,
                            "OR" => a || b,
                            "XOR" => a != b,
                            _ => panic!("bad op {}", gate.op),
                        }),
                    );
                }
            }
            updated += 1;
        }
    }

    let mut out_registers = registers
        .keys()
        .filter(|k| k.starts_with("z"))
        .collect::<Vec<_>>();
    out_registers.sort();
    loop {
        let last = out_registers.last().unwrap();
        if !registers.get(*last).unwrap().unwrap() {
            out_registers.pop();
        } else {
            break;
        }
    }
    out_registers.reverse();
    let mut v = 0;
    for r in out_registers {
        v *= 2;
        v += registers.get(r).unwrap().unwrap() as u64;
    }
    Some(v)
}

fn input_to_dot(input: &str) -> String {
    let mut dot = String::new();
    dot += "digraph circuit {\n";
    let parts = input.split("\n\n");
    let gates = parts.collect::<Vec<_>>()[1];
    let mut op = 0;
    for line in gates.lines() {
        let elts: Vec<_> = line.split(" ").collect();
        dot += &format!("{} [label=\"{}\"];\n", op, elts[1]);
        dot += &format!("{} -> {};\n", elts[0], op);
        dot += &format!("{} -> {};\n", elts[2], op);
        dot += &format!("{} -> {};\n", op, elts[4]);
        op += 1;
    }
    dot += "}\n";
    dot
}

pub fn part_two(input: &str) -> Option<u64> {
    fs::write("/tmp/24.dot", input_to_dot(input)).expect("write failed");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {}
}
