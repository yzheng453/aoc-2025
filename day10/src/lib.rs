use regex::Regex;

struct Machine{
    goal: u64,
    buttons: Vec<Vec<usize>>,
    buttons_bits: Vec<u64>,
    joltage: Vec<i64>
}

fn parse_input(input_str: &String) -> Vec<Machine> {
    let re = Regex::new(r"\[(?P<goal>[\.#]*)\](?P<buttons>(?: \(\d+(?:,\d+)*\))+) \{(?P<joltage>\d+(?:,\d+)*)\}").unwrap();
    let re_button = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
    input_str
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let goal: Vec<bool> = caps["goal"].as_bytes().iter().map(|b| match b {
                b'.' => false,
                b'#' => true,
                _ => panic!("{}", line),
            }).collect();
            
            let goal = goal.iter().enumerate().map(|(i, b)| if *b {1 << i} else {0}).sum();
            
            let buttons: Vec<Vec<usize>> = re_button.captures_iter(&caps["buttons"]).map(
                |button| {
                    let (_, [numbers]) = button.extract();
                    numbers.split(',').map(|n| n.parse().unwrap()).collect()
                }
            ).collect();
            
            let buttons_bits = buttons.iter().map(|b| b.iter().map(|n| 1 << (*n as u64)).sum()).collect();
            
            let joltage = caps["joltage"].split(',').map(|n| n.parse().unwrap()).collect();

            Machine { goal, buttons, buttons_bits, joltage }
        })
        .collect()
}

fn sch(machine: &Machine, current_state: u64, level: usize, total_presses: i64) -> Option<i64> {
    if machine.buttons_bits.len() == level {
        if current_state == machine.goal {
            return Some(total_presses);
        } else {
            return None;
        }
    }

    let not_push = sch(machine, current_state, level + 1, total_presses);
    let push = sch(machine, current_state ^ machine.buttons_bits[level], level + 1, total_presses + 1);
    
    [push, not_push].into_iter().flat_map(|o| o).min()
}

fn part1(input: Vec<Machine>) -> i64 {
    input.iter().map(|machine| {
        let presses = sch(machine, 0, 0, 0).unwrap();
        println!("{}", presses);
        presses
    }).sum()
}

fn sch2(machine: &Machine, current_state: &mut Vec<i64>, level: usize, total_presses: i64, current_min: Option<i64>) -> Option<i64> {

    if let Some(min) = current_min && min < total_presses {
        return None
    }

    if current_state.iter().all(|j| *j <= 0) {
        return Some(total_presses)
    }

    if machine.buttons.len() == level {
        return None
    }
    
    let current_button = &machine.buttons[level];
    
    let max_presses = current_button.iter().map(|j| current_state[*j]).max().unwrap_or(0);
    let mut result = None;
    for presses in 0..=max_presses {
        current_button.iter().for_each(|j| current_state[*j] -= presses);
        let current_min = sch2(machine, current_state, level + 1, total_presses + presses, result);
        current_button.iter().for_each(|j| current_state[*j] += presses);
        if let Some(c) = current_min {
            if result.is_none_or(|prev| prev > c) {
                result = current_min;
            }
        }
    }
    result
}

fn part2(input: Vec<Machine>) -> i64 {
    input.iter().map(|machine| {
        let mut state = machine.joltage.clone();
        let presses = sch2(machine, &mut state, 0, 0, None).unwrap();
        println!("{}", presses);
        presses
    }).sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::parse_input;

    #[test_case("input/sample_input.txt", 7)]
    #[test_case("input/input.txt", 507)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part1(input);

        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input.txt", 33)]
    #[test_case("input/input.txt", 0)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part2(input);

        assert_eq!(result, epxected);
    }

}
