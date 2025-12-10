use regex::Regex;

struct Machine{
    goal: u64,
    buttons: Vec<u64>,
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
            
            let buttons = re_button.captures_iter(&caps["buttons"]).map(
                |button| {
                    let (_, [numbers]) = button.extract();
                    numbers.split(',').map(|n| 1 << n.parse::<u64>().unwrap()).sum()
                }
            ).collect();
            
            let joltage = caps["joltage"].split(',').map(|n| n.parse().unwrap()).collect();

            Machine { goal, buttons, joltage }
        })
        .collect()
}

fn sch(machine: &Machine, current_state: u64, level: usize, total_presses: i64) -> Option<i64> {
    if machine.buttons.len() == level {
        if current_state == machine.goal {
            return Some(total_presses);
        } else {
            return None;
        }
    }

    let not_push = sch(machine, current_state, level + 1, total_presses);
    let push = sch(machine, current_state ^ machine.buttons[level], level + 1, total_presses + 1);
    
    [push, not_push].into_iter().flat_map(|o| o).min()
}

fn part1(input: Vec<Machine>) -> i64 {
    input.iter().map(|machine| {
        let presses = sch(machine, 0, 0, 0).unwrap();
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
    #[test_case("input/input.txt", 0)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part1(input);

        assert_eq!(result, epxected);
    }

}
