advent_of_code::solution!(13);
use sscanf::sscanf;
use std::cmp;

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut total_cost = 0;
    loop {
        let (ax, ay) = sscanf!(lines.next().unwrap(), "Button A: X+{u64}, Y+{u64}").unwrap();
        let (bx, by) = sscanf!(lines.next().unwrap(), "Button B: X+{u64}, Y+{u64}").unwrap();
        let (px, py) = sscanf!(lines.next().unwrap(), "Prize: X={u64}, Y={u64}").unwrap();

        let (mut x, mut y, mut b, mut min_cost) = (0, 0, 0, u64::max_value());
        while x < px && y < py {
            if (px - x) % ax == 0 && py - y == ay * (px - x) / ax {
                min_cost = cmp::min(min_cost, b + 3 * (px - x) / ax);
            }
            (b, x, y) = (b + 1, x + bx, y + by);
        }
        if min_cost < u64::max_value() {
            total_cost += min_cost;
        }

        if lines.next() == None {
            break;
        }
    }
    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut lines = input.lines();
    let mut total_cost = 0;
    loop {
        let (ax, ay) = sscanf!(lines.next().unwrap(), "Button A: X+{i64}, Y+{i64}").unwrap();
        let (bx, by) = sscanf!(lines.next().unwrap(), "Button B: X+{i64}, Y+{i64}").unwrap();
        let (mut px, mut py) = sscanf!(lines.next().unwrap(), "Prize: X={i64}, Y={i64}").unwrap();
        (px, py) = (px + 10000000000000, py + 10000000000000);

        use z3::ast::{Ast, Int};
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&ctx);
        let za = &Int::new_const(&ctx, "a");
        let zb = &Int::new_const(&ctx, "b");
        let zpx = &Int::from_i64(&ctx, px);
        let zpy = &Int::from_i64(&ctx, py);
        let zzero = &Int::from_i64(&ctx, 0);
        solver.assert(&(za * ax + zb * bx)._eq(zpx));
        solver.assert(&(za * ay + zb * by)._eq(zpy));
        solver.assert(&za.ge(zzero));
        solver.assert(&zb.ge(zzero));
        if solver.check() == z3::SatResult::Sat {
            let model = solver.get_model().unwrap();
            let va = model.eval(za, true).unwrap().as_u64().unwrap();
            let vb = model.eval(zb, true).unwrap().as_u64().unwrap();
            total_cost += 3 * va + vb;
        }

        if lines.next() == None {
            break;
        }
    }
    Some(total_cost as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
