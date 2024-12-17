advent_of_code::solution!(14);
use sscanf::sscanf;

pub fn part_one(input: &str) -> Option<u64> {
    let mut bots = vec![];
    for line in input.lines() {
        let (px, py, vx, vy) = sscanf!(line, "p={i64},{i64} v={i64},{i64}").unwrap();
        bots.push(((px, py), (vx, vy)));
    }
    let (xdim, ydim) = (101, 103);
    for _t in 0..100 {
        for i in 0..bots.len() {
            bots[i].0 .0 += bots[i].1 .0;
            bots[i].0 .1 += bots[i].1 .1;
        }
    }
    let mut quads = vec![0, 0, 0, 0];
    for bot in bots {
        let (x, y) = (
            ((bot.0 .0 % xdim) + xdim) % xdim,
            ((bot.0 .1 % ydim) + ydim) % ydim,
        );
        if x < xdim / 2 {
            if y < ydim / 2 {
                quads[0] += 1;
            } else if y > ydim / 2 {
                quads[1] += 1;
            }
        } else if x > xdim / 2 {
            if y < ydim / 2 {
                quads[2] += 1;
            } else if y > ydim / 2 {
                quads[3] += 1;
            }
        }
    }
    Some(quads.into_iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut bots = vec![];
    for line in input.lines() {
        let (px, py, vx, vy) = sscanf!(line, "p={i64},{i64} v={i64},{i64}").unwrap();
        bots.push(((px, py), (vx, vy)));
    }
    let (xdim, ydim) = (101i64, 103i64);
    for _t in 0..10000 {
        let mut grid = (".".repeat(xdim as usize) + "\n")
            .repeat(ydim as usize)
            .into_bytes();
        for i in 0..bots.len() {
            bots[i].0 .0 += bots[i].1 .0;
            bots[i].0 .1 += bots[i].1 .1;
            let (x, y) = (
                ((bots[i].0 .0 % xdim) + xdim) % xdim,
                ((bots[i].0 .1 % ydim) + ydim) % ydim,
            );
            grid[(x + y * (xdim + 1)) as usize] = b'*';
        }
        let tree = String::from_utf8(grid).unwrap();
        if tree.contains("********************") {
            println!("{}:\n{}", _t, tree);
            return Some(_t + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {}
}
