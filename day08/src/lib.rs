use std::{
    array, cmp::Ordering, collections::{BTreeMap, BTreeSet, BinaryHeap}, usize
};

use regex::Regex;

type Point = [i64; 3];

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Edge {
    d_sq: i64,
    x: usize, y: usize
}

fn parse_input(input_str: &String) -> Vec<Point> {
    input_str
        .lines()
        .map(|line| {
            let mut iter = line.split(',')
                .map(|n| n.parse().unwrap());
            array::from_fn(|_| iter.next().unwrap())
        })
        .collect()
}

fn find_parent(parent: &mut Vec<usize>, n: usize) -> usize {
    let mut p = n;
    while parent[p] != p {
        p = parent[p];
    }
    let mut q = n;
    while parent[q] != p {
        (q, parent[q]) = (parent[q], p);
    }
    p
}

fn union(parent: &mut Vec<usize>, a: usize, b: usize) {
    let p_a = find_parent(parent, a);
    let p_b = find_parent(parent, b);
    parent[p_a] = p_b;
}

fn part1(input: Vec<Point>, n_connect: u64) -> i64 {
    let mut edges = Vec::new();
    for (i, x) in input.iter().enumerate() {
        for (j, y) in input.iter().enumerate().skip(i + 1) {
            let dist_sq = (0..3).map(|d| (x[d] - y[d]) * (x[d] - y[d])).sum();
            edges.push(Edge{d_sq: dist_sq, x: i, y: j});
        }
    }
    edges.sort();
    let mut parent: Vec<usize> = (0..input.len()).collect();
    let mut count = n_connect;
    let mut e_iter = edges.iter();
    while count > 0 {
        let e = e_iter.next().unwrap();
        /*if find_parent(&mut parent, e.x) == find_parent(&mut parent, e.y) {
            continue;
        }*/
        union(&mut parent, e.x, e.y);
        count -= 1;
    }

    let mut circuits: Vec<i64> = (0..input.len()).map(|_| 0).collect();
    for i in 0..input.len() {
        let p = find_parent(&mut parent, i);
        circuits[p] += 1;
    }
    
    let mut ordered: Vec<usize> = (0..input.len()).collect();
    ordered.sort_by(|a, b| {
        circuits[*b].cmp(&circuits[*a])
    });
    (0..3).map(|i| circuits[ordered[i]]).product()
}

fn part2(input: Vec<Point>) -> i64 {
    let mut edges = Vec::new();
    for (i, x) in input.iter().enumerate() {
        for (j, y) in input.iter().enumerate().skip(i + 1) {
            let dist_sq = (0..3).map(|d| (x[d] - y[d]) * (x[d] - y[d])).sum();
            edges.push(Edge{d_sq: dist_sq, x: i, y: j});
        }
    }
    edges.sort();
    let mut parent: Vec<usize> = (0..input.len()).collect();
    let mut count = input.len() - 1;
    let mut e_iter = edges.iter();
    while count > 0 {
        let e = e_iter.next().unwrap();
        if find_parent(&mut parent, e.x) == find_parent(&mut parent, e.y) {
            continue;
        }
        union(&mut parent, e.x, e.y);
        count -= 1;
        if count == 0 {
            return input[e.x][0] * input[e.y][0];
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::parse_input;

    #[test_case("input/sample_input.txt", 10, 40)]
    #[test_case("input/input.txt", 1000, 46398)]
    fn test_part1(input_file: &str, n_connect: u64, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part1(input, n_connect);

        assert_eq!(result, epxected);
    }
    
    #[test_case("input/sample_input.txt", 25272)]
    #[test_case("input/input.txt", 8141888143)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part2(input);

        assert_eq!(result, epxected);
    }
}
