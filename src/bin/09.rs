advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks: Vec<isize> = vec![];
    let mut id = 0;
    input.trim().chars().into_iter().fold(true, |data, x| {
        let size = x.to_digit(10).unwrap();
        if data {
            for _i in 0..size {
                blocks.push(id);
            }
            id += 1;
        } else {
            for _i in 0..size {
                blocks.push(-1);
            }
        }
        !data
    });

    let mut cursor = blocks.len() - 1;
    let mut free = 0;
    loop {
        while blocks[free] != -1 && free < blocks.len() {
            free += 1;
        }
        while blocks[cursor] == -1 && cursor > 0 {
            cursor -= 1;
        }
        if free >= cursor {
            break;
        }
        blocks[free] = blocks[cursor];
        blocks[cursor] = -1;
    }

    let mut checksum = 0;
    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            checksum += blocks[i] * i as isize;
        }
    }
    Some(checksum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks: Vec<isize> = vec![];
    let mut sizes: Vec<u32> = vec![];
    let mut id = 0;
    input.trim().chars().into_iter().fold(true, |data, x| {
        let size = x.to_digit(10).unwrap();
        if data {
            for _i in 0..size {
                blocks.push(id);
                sizes.push(size);
            }
            id += 1;
        } else {
            for _i in 0..size {
                blocks.push(-1);
                sizes.push(size);
            }
        }
        !data
    });

    let mut cursor = blocks.len() - 1;
    loop {
        while blocks[cursor] == -1 {
            cursor -= 1;
        }
        if blocks[cursor] == 0 {
            break;
        }
        let mut free = 0;
        while free < cursor && (blocks[free] != -1 || sizes[free] < sizes[cursor]) {
            free += 1;
        }
        if free == cursor {
            while blocks[cursor] == blocks[free] {
                cursor -= 1;
            }
            continue;
        }

        for i in sizes[cursor]..sizes[free] {
            sizes[free + i as usize] = sizes[free] - sizes[cursor];
        }
        for _i in 0..sizes[cursor] {
            blocks[free] = blocks[cursor];
            blocks[cursor] = -1;
            cursor -= 1;
            free += 1;
        }
    }

    let mut checksum = 0;
    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            checksum += blocks[i] * i as isize;
        }
    }
    Some(checksum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
