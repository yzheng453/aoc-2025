
fn part1(input_str: String) -> i32 {
    let (_, count) = input_str.split_whitespace()
        .map(|line| {
            let direction = &line
                .chars()
                .nth(0)
                .expect(&format!("Unknown line {}", line));
            let clicks: i32 = (&line[1..])
                .parse::<i32>()
                .expect(&format!("Unknown line {}", line));
            let rotation = match direction {
                'L' => -clicks,
                'R' => clicks,
                _ => panic!("Unknown line {}", line),
            };
            rotation
        })
        .fold((50, 0), |(position, count), rotation| {
            let new_position = (position + rotation).rem_euclid(100);
            (new_position.rem_euclid(100), count + (if position == 0 {1} else {0}))
        });

    count
}

pub fn part2(input_str: String) -> i32 {
    let (_, count) = input_str.split_whitespace()
        .map(|line| {
            let direction = &line
                .chars()
                .nth(0)
                .expect(&format!("Unknown line {}", line));
            let clicks: i32 = (&line[1..])
                .parse::<i32>()
                .expect(&format!("Unknown line {}", line));
            let rotation = match direction {
                'L' => -clicks,
                'R' => clicks,
                _ => panic!("Unknown line {}", line),
            };
            rotation
        })
        .fold((50, 0), |(position, count), rotation| {
            let new_position = position + rotation;

            let n_crosses = if rotation > 0 {
                new_position.div_euclid(100) - position.div_euclid(100)
            } else {
                (position - 1).div_euclid(100) - (new_position - 1).div_euclid(100)
            };
            
            (new_position.rem_euclid(100), count + n_crosses)
        });

    count
}


#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::{part1, part2};
    
    #[test_case("input/sample_input.txt", 3)]
    #[test_case("input/input.txt", 1018)]
    fn test_part_1(input_file: &str, epxected: i32) {
        let input_str = fs::read_to_string(input_file).unwrap();
        
        let result = part1(input_str);
        
        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input.txt", 6)]
    #[test_case("input/custom_sample.txt", 3)]
    #[test_case("input/input.txt", 5815)]
    fn test_part_2(input_file: &str, epxected: i32) {
        let input_str = fs::read_to_string(input_file).unwrap();
        
        let result = part2(input_str);
        
        assert_eq!(result, epxected);
    }
}
