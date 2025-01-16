advent_of_code::solution!(23);
use std::collections::BTreeSet;
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
    let mut edges = HashMap::new();
    let mut nodes = HashSet::new();
    let mut cliques: HashSet<BTreeSet<&str>> = HashSet::new();
    for conn in input.lines() {
        let cs: Vec<_> = conn.split('-').collect();
        edges.entry(cs[0]).or_insert(vec![]).push(cs[1]);
        edges.entry(cs[1]).or_insert(vec![]).push(cs[0]);
        nodes.insert(cs[0]);
        nodes.insert(cs[1]);
        cliques.insert(BTreeSet::from([cs[0], cs[1]]));
    }
    while !cliques.is_empty() {
        let mut next_cliques = HashSet::new();
        for c in &cliques {
            for n in &nodes {
                if c.contains(n) {
                    continue;
                }
                let n_edges = edges.get(n).unwrap();
                if c.iter().all(|e| *e == *n || n_edges.contains(e)) {
                    let mut next_clique = c.clone();
                    next_clique.insert(&n);
                    next_cliques.insert(next_clique);
                }
            }
        }
        if next_cliques.is_empty() {
            return Some(
                cliques
                    .into_iter()
                    .next()
                    .unwrap()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        cliques = next_cliques;
    }
    None
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
