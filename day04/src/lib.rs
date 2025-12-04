use std::usize;

use regex::Regex;

#[derive(PartialEq)]
enum Spot {
    Roll,
    Empty
}

fn parse_input(input_str: &String) -> Vec<Vec<Spot>> {
    input_str
        .split_whitespace()
        .map(|line| line.as_bytes().iter().map(|b| match *b {
            b'.' => Spot::Empty,
            b'@' => Spot::Roll,
            _ => panic!("Unrecognizeable line {}", line),
        }).collect())
        .collect()
}

fn part1(input: Vec<Vec<Spot>>) -> u64 {
    let mut count = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] != Spot::Roll {
                continue;
            }

            let adj_roll_count = (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .flat_map(|(dx, dy)| TryInto::<usize>::try_into((x as i32 + dx)).and_then(|nx| (y as i32 + dy).try_into().and_then(|ny: usize| Ok((nx, ny)))))
                .flat_map(|(nx, ny)| input.get(nx).and_then(|nrow| nrow.get(ny)))
                .filter(|v| **v == Spot::Roll)
                .count();
            
            if adj_roll_count <= 4 {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: impl Iterator<Item = Vec<u8>>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{parse_input, part1, part2};

    #[test_case("input/sample_input.txt", 13)]
    #[test_case("input/input.txt", 1424)]
    fn test_part1(input_file: &str, epxected: u64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part1(input);

        assert_eq!(result, epxected);
    }

    /*
    #[test_case("input/sample_input.txt", 3121910778619)]
    #[test_case("input/input.txt", 170997883706617)]
    fn test_part2(input_file: &str, epxected: u64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part2(input);

        assert_eq!(result, epxected);
    }
    */
}
