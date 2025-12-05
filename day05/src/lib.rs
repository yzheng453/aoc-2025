use std::usize;

use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Interval(i64, i64);

fn parse_input(input_str: &String) -> (Vec<Interval>, Vec<i64>) {
    let re: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    let intervals = input_str
        .split_whitespace()
        .take_while(|s| s.contains('-'))
        .map(|s| {
            re.captures(s)
                .map(|captures| {
                    Interval(
                        captures.get(1).unwrap().as_str().parse().unwrap(),
                        captures.get(2).unwrap().as_str().parse().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();

    let ingredients = input_str
        .split_whitespace()
        .skip_while(|s| s.contains('-'))
        .map(|s| s.parse().unwrap())
        .collect();
    (intervals, ingredients)
}

fn compact_intervals(mut input_intervals: Vec<Interval>) -> Vec<Interval> {
    input_intervals.sort();
    let mut last = input_intervals[0];
    let mut result = Vec::new();
    let mut i = 1;
    while i < input_intervals.len() {
        let current = input_intervals[i];
        if current.0 <= last.1 {
            if current.1 > last.1 {
                last.1 = current.1;
            }
        } else {
            result.push(last);
            last = current;
        }
        i += 1;
    }
    result.push(last);
    result
}

fn part1(input: (Vec<Interval>, Vec<i64>)) -> i64 {
    let (input_intervals, mut ingredients) = input;
    let interval = compact_intervals(input_intervals);
    ingredients.sort();
    let mut interval_index = 0;
    let mut count = 0;
    for ingredient in ingredients {
        while interval_index < interval.len() && interval[interval_index].1 < ingredient {
            interval_index += 1;
        }
        if let Some(i) = interval.get(interval_index) {
            if i.0 <= ingredient {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: (Vec<Interval>, Vec<i64>)) -> i64 {
    let (input_intervals, _) = input;
    let interval = compact_intervals(input_intervals);
    interval.iter().map(|i| i.1 - i.0 + 1).sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{parse_input, part1};

    #[test_case("input/sample_input.txt", 3)]
    #[test_case("input/input.txt", 513)]
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
        let result = crate::part2(input);

        assert_eq!(result, epxected);
    }
}
