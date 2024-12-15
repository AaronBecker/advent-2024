advent_of_code::solution!(15);

fn push(
    grid: &mut Vec<Vec<char>>,
    pos: (isize, isize),
    delta: (isize, isize),
) -> Option<(isize, isize)> {
    let (mut x, mut y) = pos;
    assert!(grid[y as usize][x as usize] == '@');
    while x >= 0
        && y >= 0
        && x < grid[0].len() as isize
        && y < grid.len() as isize
        && grid[y as usize][x as usize] != '#'
    {
        if grid[y as usize][x as usize] == '.' {
            return Some((x, y));
        }
        (x, y) = (x + delta.0, y + delta.1);
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];
    let mut parse_moves = false;
    let (mut cx, mut cy) = (0, 0);
    for line in input.lines() {
        if line.is_empty() {
            parse_moves = true;
            continue;
        }
        if parse_moves {
            moves.extend(line.chars());
        } else {
            let l: Vec<char> = line.chars().collect();
            if let Some(x) = l.iter().position(|c| *c == '@') {
                (cx, cy) = (x as isize, grid.len() as isize);
            }
            grid.push(l);
        }
    }

    assert!(grid[cy as usize][cx as usize] == '@');
    for m in moves {
        let delta = match m {
            '^' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            _ => panic!("bad instruction {}", m),
        };
        if let Some((mut dx, mut dy)) = push(&mut grid, (cx, cy), delta) {
            loop {
                grid[dy as usize][dx as usize] =
                    grid[(dy - delta.1) as usize][(dx - delta.0) as usize];
                if grid[dy as usize][dx as usize] == '@' {
                    grid[cy as usize][cx as usize] = '.';
                    (cx, cy) = (dx, dy);
                    break;
                }
                (dx, dy) = (dx - delta.0, dy - delta.1);
            }
        }
        assert!(grid[cy as usize][cx as usize] == '@');
    }

    let mut gps_sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                gps_sum += x + 100 * y;
            }
        }
    }
    Some(gps_sum as u64)
}

fn can_push2(grid: &mut Vec<Vec<char>>, pos: (isize, isize), delta: (isize, isize)) -> bool {
    let glyph = grid[pos.1 as usize][pos.0 as usize];
    if glyph == '#' || glyph == '.' {
        return glyph == '.';
    }
    if glyph == '@' {
        return can_push2(grid, (pos.0 + delta.0, pos.1 + delta.1), delta);
    }
    if glyph == '[' || glyph == ']' {
        if delta.1 == 0 {
            return can_push2(grid, (pos.0 + delta.0, pos.1 + delta.1), delta);
        }
        let other_half = if glyph == '[' { 1 } else { -1 };
        return can_push2(grid, (pos.0, pos.1 + delta.1), delta)
            && can_push2(grid, (pos.0 + other_half, pos.1 + delta.1), delta);
    }
    false
}

fn do_push2(grid: &mut Vec<Vec<char>>, pos: (isize, isize), delta: (isize, isize)) -> () {
    let glyph = grid[pos.1 as usize][pos.0 as usize];
    assert_ne!(glyph, '#');
    if glyph == '.' {
        return;
    }
    if glyph == '@' {
        do_push2(grid, (pos.0 + delta.0, pos.1 + delta.1), delta);
        grid[pos.1 as usize][pos.0 as usize] = '.';
        grid[(pos.1 + delta.1) as usize][(pos.0 + delta.0) as usize] = '@';
    }
    if glyph == '[' || glyph == ']' {
        if delta.1 == 0 {
            do_push2(grid, (pos.0 + delta.0, pos.1 + delta.1), delta);
            grid[pos.1 as usize][pos.0 as usize] =
                grid[(pos.1 - delta.1) as usize][(pos.0 - delta.0) as usize];
            grid[(pos.1 + delta.1) as usize][(pos.0 + delta.0) as usize] = glyph;
        } else {
            let other_half = if glyph == '[' { 1 } else { -1 };
            do_push2(grid, (pos.0, pos.1 + delta.1), delta);
            do_push2(grid, (pos.0 + other_half, pos.1 + delta.1), delta);
            grid[(pos.1 + delta.1) as usize][(pos.0 + delta.0) as usize] = glyph;
            grid[(pos.1 + delta.1) as usize][(pos.0 + other_half + delta.0) as usize] =
                if glyph == '[' { ']' } else { '[' };
            grid[pos.1 as usize][pos.0 as usize] = '.';
            grid[pos.1 as usize][(pos.0 + other_half) as usize] = '.';
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];
    let mut parse_moves = false;
    let (mut cx, mut cy) = (0, 0);
    for line in input.lines() {
        if line.is_empty() {
            parse_moves = true;
            continue;
        }
        if parse_moves {
            moves.extend(line.chars());
        } else {
            let l: Vec<char> = line.chars().collect();
            let mut wide_line = vec![];
            for c in l.iter() {
                match c {
                    'O' => {
                        wide_line.push('[');
                        wide_line.push(']');
                    }
                    '@' => {
                        (cx, cy) = (wide_line.len() as isize, grid.len() as isize);
                        wide_line.push('@');
                        wide_line.push('.');
                    }
                    _ => {
                        wide_line.push(*c);
                        wide_line.push(*c);
                    }
                }
            }
            grid.push(wide_line);
        }
    }

    assert!(grid[cy as usize][cx as usize] == '@');
    for m in moves {
        let delta = match m {
            '^' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            _ => panic!("bad instruction {}", m),
        };
        if can_push2(&mut grid, (cx, cy), delta) {
            do_push2(&mut grid, (cx, cy), delta);
            (cx, cy) = (cx + delta.0, cy + delta.1);
        } else {
        }
        assert!(grid[cy as usize][cx as usize] == '@');
    }

    let mut gps_sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '[' {
                gps_sum += x + 100 * y;
            }
        }
    }
    Some(gps_sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
",
        );
        assert_eq!(result, Some(2028));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
",
        );
        assert_eq!(result, Some(618));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
