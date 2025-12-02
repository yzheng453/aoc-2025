use regex::Regex;

fn parse_input(input_str: &String) -> impl Iterator<Item = (i64, i64)> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    input_str.split(',').map(move |line| {
        let Some(caps) = re.captures(line) else {panic!("Unrecognizeable line {}", line)};
        let Some(left) = caps.get(1) else {panic!("Unrecognizeable line {}", line)};
        let Some(right) = caps.get(2) else {panic!("Unrecognizeable line {}", line)};

        (left.as_str().parse().unwrap(), right.as_str().parse().unwrap())
    })
}

fn is_valid(mut n: i64) -> bool {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    if digits.len() % 2 != 0 {
        return true;
    }
    let h = digits.len() / 2;
    for i in 0..h {
        if digits[i] != digits[i + h] {
            return true;
        }
    }
    false
}

fn part1(input: impl Iterator<Item = (i64, i64)>) -> i64 {
    input.map(|(l, r)| {
        (l..=r).into_iter().filter(|&n| !is_valid(n)).sum::<i64>()
    }).into_iter().sum()
}

pub fn part2(input_str: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{is_valid, parse_input, part1, part2};

    #[test_case(11, false)]
    #[test_case(101, true)]
    #[test_case(446446, false)]
    fn test_is_invalid(n: i64, expected: bool) {
        assert_eq!(is_valid(n), expected)
    }

    #[test_case("input/sample_input.txt", 1227775554)]
    #[test_case("input/input.txt", 23039913998)]
    fn test_part_1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part1(input);

        assert_eq!(result, epxected);
    }

    /* 
    #[test_case("input/sample_input.txt", 6)]
    #[test_case("input/custom_sample.txt", 3)]
    #[test_case("input/input.txt", 5815)]
    fn test_part_2(input_file: &str, epxected: i32) {
        let input_str = fs::read_to_string(input_file).unwrap();

        let result = part2(input_str);

        assert_eq!(result, epxected);
    }
*/
}
