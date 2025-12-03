use regex::Regex;

fn parse_input(input_str: &String) -> impl Iterator<Item = Vec<u8>> {
    input_str
        .split_whitespace()
        .map(|line| line.as_bytes().iter().map(|b| *b - b'0').collect())
}

fn part1(input: impl Iterator<Item = Vec<u8>>) -> u64 {
    input.map(|v| joltage(&v[..], 2).unwrap()).into_iter().sum()
}

fn joltage(s: &[u8], digits: usize) -> Option<u64> {
    if s.len() < digits {
        return None;
    }
    if digits == 1 {
        return s.iter().max().map(|u| *u as u64);
    }
    let max_first_digit = s.iter().rev().skip(digits - 1).max().unwrap().clone();
    let (first_occur_max_first_digit, _) = s
        .iter()
        .enumerate()
        .find(|(_, u)| **u == max_first_digit)
        .unwrap();
    
    let n = max_first_digit as u64 * 10_u64.pow(digits as u32 - 1);
    joltage(&s[first_occur_max_first_digit + 1..], digits - 1)
        .map(|l| n + l)
}

fn part2(input: impl Iterator<Item = Vec<u8>>) -> u64 {
    input.map(|v| joltage(&v[..], 12).unwrap()).into_iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{joltage, parse_input, part1, part2};

    #[test_case(vec!(9,8,7,6,5,4,3,2,1,1,1,1,1,1,1), 98)]
    #[test_case(vec!(8,1,1,1,1,1,1,1,1,1,1,1,1,1,9), 89)]
    fn test_joltage(n: Vec<u8>, expected: u64) {
        assert_eq!(joltage(&n[..], 2).unwrap(), expected)
    }

    #[test_case("input/sample_input.txt", 357)]
    #[test_case("input/input.txt", 17207)]
    fn test_part1(input_file: &str, epxected: u64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part1(input);

        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input.txt", 3121910778619)]
    #[test_case("input/input.txt", 170997883706617)]
    fn test_part2(input_file: &str, epxected: u64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part2(input);

        assert_eq!(result, epxected);
    }
}
