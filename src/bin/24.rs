advent_of_code::solution!(24);
use std::fs;

pub fn part_one(_input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
