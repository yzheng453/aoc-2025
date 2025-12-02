use regex::Regex;

fn parse_input(input_str: &String) -> impl Iterator<Item = (i64, i64)> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    input_str.split(',').map(move |line| {
        let Some(caps) = re.captures(line) else {
            panic!("Unrecognizeable line {}", line)
        };
        let Some(left) = caps.get(1) else {
            panic!("Unrecognizeable line {}", line)
        };
        let Some(right) = caps.get(2) else {
            panic!("Unrecognizeable line {}", line)
        };

        (
            left.as_str().parse().unwrap(),
            right.as_str().parse().unwrap(),
        )
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
    input
        .map(|(l, r)| (l..=r).into_iter().filter(|&n| !is_valid(n)).sum::<i64>())
        .into_iter()
        .sum()
}

fn is_valid_part2(mut n: i64) -> bool {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    'l_loop: for l in 1..digits.len() {
        if digits.len() % l != 0 {
            continue 'l_loop;
        }
        let h = digits.len() / l;
        for i in 0..l {
            for j in 1..h {
                if digits[i] != digits[i + l * j] {
                    continue 'l_loop;
                }
            }
        }
        return false;
    }
    true
}

fn part2(input: impl Iterator<Item = (i64, i64)>) -> i64 {
    input
        .map(|(l, r)| (l..=r).into_iter().filter(|&n| !is_valid_part2(n)).sum::<i64>())
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{is_valid, is_valid_part2, parse_input, part1, part2};

    #[test_case(11, false)]
    #[test_case(101, true)]
    #[test_case(446446, false)]
    fn test_is_valid(n: i64, expected: bool) {
        assert_eq!(is_valid(n), expected)
    }

    #[test_case("input/sample_input.txt", 1227775554)]
    #[test_case("input/input.txt", 23039913998)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part1(input);

        assert_eq!(result, epxected);
    }
    
    #[test_case(11, false)]
    #[test_case(101, true)]
    #[test_case(446446, false)]
    #[test_case(121212, false)]
    #[test_case(1111111, false)]
    fn test_is_valid_part_2(n: i64, expected: bool) {
        assert_eq!(is_valid_part2(n), expected)
    }

    #[test_case("input/sample_input.txt", 4174379265)]
    #[test_case("input/input.txt", 35950619148)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part2(input);

        assert_eq!(result, epxected);
    }
}
