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

fn shift_coord(c: (usize, usize), d: (i32, i32)) -> Option<(usize, usize)> {
    let nx: usize = (c.0 as i32 + d.0).try_into().ok()?;
    let ny: usize = (c.1 as i32 + d.1).try_into().ok()?;
    Some((nx, ny))
}

fn try_get(coord: (usize, usize), grid: &Vec<Vec<Spot>>) -> Option<&Spot> {
    let row = grid.get(coord.0)?;
    row.get(coord.1)
}

fn part1(input: Vec<Vec<Spot>>) -> u64 {
    let mut count = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] != Spot::Roll {
                continue;
            }

            let adj_roll_count = (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .flat_map(|d| shift_coord((x, y), d))
                .flat_map(|c| try_get(c, &input))
                .filter(|v| **v == Spot::Roll)
                .count();
            
            if adj_roll_count <= 4 {
                count += 1;
            }
        }
    }
    count
}

fn part2(mut grid: Vec<Vec<Spot>>) -> i32 {
    let mut count = 0;
    loop {
        let last_removed = try_remove(&mut grid);
        count += last_removed;
        if last_removed == 0 {
            break;
        }
    }
    
    count
}

fn try_remove(grid: &mut Vec<Vec<Spot>>) -> i32 {
    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] != Spot::Roll {
                continue;
            }

            let adj_roll_count = (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .flat_map(|d| shift_coord((x, y), d))
                .flat_map(|c| try_get(c, &grid))
                .filter(|v| **v == Spot::Roll)
                .count();
            
            if adj_roll_count <= 4 {
                grid[x][y] = Spot::Empty;
                count += 1;
            }
        }
    }
    count
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

    #[test_case("input/sample_input.txt", 43)]
    #[test_case("input/input.txt", 8727)]
    fn test_part2(input_file: &str, epxected: i32) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = part2(input);

        assert_eq!(result, epxected);
    }
}
