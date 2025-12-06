use std::usize;

use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Op {
    Plus, Multi
}

fn parse_input(input_str: &String) -> (Vec<Vec<i64>>, Vec<Op>) {
    let numbers = input_str
        .lines()
        .take_while(|s| !s.contains('+') && ! s.contains('*'))
        .map(|s| 
            s.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        )
        .collect();

    let ops = input_str
        .lines()
        .skip_while(|s| !s.contains('+') && ! s.contains('*'))
        .next().unwrap()
        .split_whitespace()
        .map(|s| if s.contains('+') {Op::Plus} else {if s.contains('*') {Op::Multi} else {panic!("{}", s)}})
        .collect();
    (numbers, ops)
}

fn part1(input: (Vec<Vec<i64>>, Vec<Op>)) -> i64 {
    let (numbers, ops) = input;
    ops.iter().enumerate().map(|(i, op) | {
        match op {
            Op::Plus => numbers.iter().map(|n| n[i]).sum::<i64>(),
            Op::Multi => numbers.iter().map(|n| n[i]).product(),
        }
    })
    .sum()
}

/*
fn part2(input: (Vec<Interval>, Vec<i64>)) -> i64 {
    let (input_intervals, _) = input;
    let interval = compact_intervals(input_intervals);
    interval.iter().map(|i| i.1 - i.0 + 1).sum()
} */

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{parse_input, part1};
    #[test_case("input/sample_input.txt", 4277556)]
    #[test_case("input/input.txt", 4722948564882)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part1(input);

        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input.txt", 14)]
    #[test_case("input/input.txt", 339668510830757)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        //let result = crate::part2(input);

        //assert_eq!(result, epxected);
    }
}
