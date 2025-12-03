use regex::Regex;

fn parse_input(input_str: &String) -> impl Iterator<Item = Vec<u8>> {
    input_str
        .split_whitespace()
        .map(|line| line.as_bytes().iter().map(|b| *b - b'0').collect())
}

fn joltage(v: Vec<u8>) -> i64 {
    let max_first_digit = v.iter().rev().skip(1).max().unwrap().clone();
    let (first_occur_max_first_digit, _) = v
        .iter()
        .enumerate()
        .find(|(i, u)| **u == max_first_digit)
        .unwrap();
    let &max_second_digit = v
        .iter()
        .skip(first_occur_max_first_digit + 1)
        .max()
        .unwrap();
    max_first_digit as i64 * 10 + max_second_digit as i64
}

fn part1(input: impl Iterator<Item = Vec<u8>>) -> i64 {
    input.map(|v| joltage(v)).into_iter().sum()
}

fn part2(input: impl Iterator<Item = (i64, i64)>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{joltage, parse_input, part1};

    #[test_case(vec!(9,8,7,6,5,4,3,2,1,1,1,1,1,1,1), 98)]
    #[test_case(vec!(8,1,1,1,1,1,1,1,1,1,1,1,1,1,9), 89)]
    fn test_joltage(n: Vec<u8>, expected: i64) {
        assert_eq!(joltage(n), expected)
    }

    #[test_case("input/sample_input.txt", 357)]
    #[test_case("input/input.txt", 17207)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part1(input);

        assert_eq!(result, epxected);
    }

}
