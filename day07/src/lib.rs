use std::{
    collections::{BTreeMap, BTreeSet},
    usize,
};

use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Cell {
    Start,
    Splitter,
    Empty,
}

fn parse_input(input_str: &String) -> Vec<Vec<Cell>> {
    input_str
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| match b {
                    b'S' => Cell::Start,
                    b'^' => Cell::Splitter,
                    b'.' => Cell::Empty,
                    _ => panic!("Unknown char {}", b),
                })
                .collect()
        })
        .collect()
}

fn part1(input: Vec<Vec<Cell>>) -> i64 {
    let start = input
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == Cell::Start)
                .map(move |(y, _)| (x, y))
        })
        .next()
        .unwrap();
    let (_, sum) = input.iter().skip(start.0 + 1).fold(
        {
            let mut current_beam = BTreeSet::new();
            current_beam.insert(start.1);
            (current_beam, 0)
        },
        |(current_beam, sum), row| {
            let mut next_beam = BTreeSet::new();
            let splits: i64 = current_beam
                .iter()
                .map(|b| match row[*b] {
                    Cell::Empty => {
                        next_beam.insert(*b);
                        0
                    }
                    Cell::Splitter => {
                        next_beam.insert(b - 1);
                        next_beam.insert(b + 1);
                        1
                    }
                    _ => panic!(),
                })
                .sum();
            (next_beam, splits + sum)
        },
    );
    sum
}

fn part2(input: Vec<Vec<Cell>>) -> i64 {
    let timelines: BTreeMap<usize, i64> = input
        .iter()
        .fold(
            None,
            |current_beam_opt: Option<BTreeMap<usize, i64>>, row| match current_beam_opt {
                Some(current_beam) => {
                    let mut next_beam = BTreeMap::new();
                    current_beam.iter().for_each(|(b, &count)| match row[*b] {
                        Cell::Empty => {
                            next_beam
                                .entry(*b)
                                .and_modify(|v| *v += count)
                                .or_insert(count);
                        }
                        Cell::Splitter => {
                            next_beam
                                .entry(b - 1)
                                .and_modify(|v| *v += count)
                                .or_insert(count);
                            next_beam
                                .entry(b + 1)
                                .and_modify(|v| *v += count)
                                .or_insert(count);
                        }
                        Cell::Start => panic!(),
                    });
                    Some(next_beam)
                }
                None => row
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == Cell::Start)
                    .next()
                    .map(|(column_number, _)| {
                        let mut current_beam = BTreeMap::new();
                        current_beam.insert(column_number, 1);
                        current_beam
                    }),
            },
        )
        .unwrap();

    timelines.values().sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::parse_input;

    #[test_case("input/sample_input.txt", 21)]
    #[test_case("input/input.txt", 1687)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part1(input);

        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input.txt", 40)]
    #[test_case("input/input.txt", 390684413472684)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part2(input);

        assert_eq!(result, epxected);
    }
}
