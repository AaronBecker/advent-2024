advent_of_code::solution!(3);
use regex::Regex;

fn do_mul(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * c.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(do_mul(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let dont_re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    let removed_input = dont_re.replace_all(input, "");
    let dont_re = Regex::new(r"(?s)don't\(\).*$").unwrap();
    let removed_input = dont_re.replace_all(&removed_input, "");
    Some(do_mul(&removed_input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
