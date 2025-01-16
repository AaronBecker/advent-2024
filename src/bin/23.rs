advent_of_code::solution!(23);
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let mut edges = HashMap::new();
    let mut ts = HashSet::new();
    for conn in input.lines() {
        let cs: Vec<_> = conn.split('-').collect();
        edges.entry(cs[0]).or_insert(vec![]).push(cs[1]);
        edges.entry(cs[1]).or_insert(vec![]).push(cs[0]);
        for n in [cs[0], cs[1]] {
            if n.starts_with("t") {
                ts.insert(n);
            }
        }
    }
    let mut triangles = HashSet::new();
    for t in ts {
        for t2 in edges.get(t).unwrap() {
            if t == *t2 {
                continue;
            }
            for t3 in edges.get(t2).unwrap() {
                if *t3 != *t2 && *t3 != t && edges.get(t3).unwrap().contains(&t) {
                    let mut triangle = vec![t, t2, t3];
                    triangle.sort();
                    triangles.insert(triangle);
                }
            }
        }
    }
    Some(triangles.len() as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    // This approach depends on the fact that the input nodes all have the same degree.
    let mut edges = HashMap::new();
    let mut nodes = HashSet::new();
    for conn in input.lines() {
        let cs: Vec<_> = conn.split('-').collect();
        edges.entry(cs[0]).or_insert(vec![]).push(cs[1]);
        edges.entry(cs[1]).or_insert(vec![]).push(cs[0]);
        nodes.insert(cs[0]);
        nodes.insert(cs[1]);
    }
    let mut triangles = HashSet::new();
    for t in nodes {
        for t2 in edges.get(t).unwrap() {
            if t == *t2 {
                continue;
            }
            for t3 in edges.get(t2).unwrap() {
                if *t3 != *t2 && *t3 != t && edges.get(t3).unwrap().contains(&t) {
                    let mut triangle = vec![t, t2, t3];
                    triangle.sort();
                    triangles.insert(triangle);
                }
            }
        }
    }
    let mut t_count: HashMap<&str, i32> = HashMap::new();
    for t in &triangles {
        *t_count.entry(t[0]).or_insert(0) += 1;
        *t_count.entry(t[1]).or_insert(0) += 1;
        *t_count.entry(t[2]).or_insert(0) += 1;
    }
    let max_t = *t_count.values().max().unwrap();
    let mut max_clique = HashSet::new();
    for t in triangles {
        if t_count[t[0]] == max_t && t_count[t[1]] == max_t && t_count[t[2]] == max_t {
            max_clique.insert(t[0]);
            max_clique.insert(t[1]);
            max_clique.insert(t[2]);
        }
    }
    let mut c_vec = max_clique.into_iter().collect::<Vec<_>>();
    c_vec.sort();
    return Some(c_vec.join(","));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_owned()));
    }
}
