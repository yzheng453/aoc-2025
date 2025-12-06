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

fn parse_input_part2(input_str: &String) -> (Vec<&[u8]>, Vec<Op>) {
    let numbers = input_str
        .lines()
        .take_while(|s| !s.contains('+') && ! s.contains('*'))
        .map(|s| s.as_bytes())
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

fn part2(input: (Vec<&[u8]>, Vec<Op>)) -> i64 {
    let (digits, ops) = input;
    let mut column = 0;
    ops.iter().map(|op| {
        let mut c = ColumnNumberIterator(&digits, &mut column);

        let result = match op {
            Op::Plus => c.sum::<i64>(),
            Op::Multi => c.product::<i64>(),
        };
        
        result
    })
    .sum()
}

struct ColumnNumberIterator<'a>(&'a Vec<&'a [u8]>, &'a mut usize);

impl Iterator for ColumnNumberIterator<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = get_vertical_num(self.0, *self.1);
        *self.1 += 1;
        result
    }
}

fn get_vertical_num(digits: &Vec<&[u8]>, column: usize) -> Option<i64> {
    if digits[0].len() <= column {
        return None
    }
    digits.iter().flat_map(|&row| match row[column] {
        b'0'..=b'9' => Some(row[column] - b'0'),
        b' ' => None,
        _ => panic!(),
    }).fold(None, |acc, c| Some(acc.unwrap_or(0) * 10 + c as i64))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{parse_input, parse_input_part2, part1};
    #[test_case("input/sample_input.txt", 4277556)]
    #[test_case("input/input.txt", 4722948564882)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part1(input);

        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input.txt", 3263827)]
    #[test_case("input/input.txt", 9581313737063)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input_part2(&input_str);
        let result = crate::part2(input);

        assert_eq!(result, epxected);
    }
}
