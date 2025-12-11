use std::collections::{HashMap, VecDeque};

type Device = [u8; 3];
const YOU: Device = [b'y', b'o', b'u'];
const OUT: Device = [b'o', b'u', b't'];

fn parse_input(input_str: &String) -> HashMap<Device, Vec<Device>> {
    input_str
        .lines()
        .map(|line| {
            let mut iter = line.split(':');
            let mut from: Device = [0; 3];
            from.copy_from_slice(iter.next().unwrap().as_bytes());
            
            let tos = iter.next().unwrap()
                .split_whitespace()
                .map(|to_str| {
                    let mut to: Device = [0; 3];
                    to.copy_from_slice(to_str.as_bytes());
                    to
                }).collect();
            (from, tos)
        })
        .collect()
}

fn part1(edges: HashMap<Device, Vec<Device>>) -> i64 {
    let mut sorted_devices: Vec<Device> = Vec::new();
    let mut in_degree: HashMap<Device, i64> = edges.keys().map(|d| (d.clone(), 0)).collect();
    assert_eq!(*in_degree.get(&YOU).unwrap(), 0);
    let mut queue = VecDeque::new();
    queue.push_back(YOU);
    sorted_devices.push(YOU);
    while let Some(d) = queue.pop_front() {
        for e in edges.get(&d).unwrap() {
            let i = in_degree.get_mut(e).unwrap();
            *i -= 1;
            if *i == 0 {
                queue.push_back(*e);
                sorted_devices.push(*e);
            }
        }
    }
    
    assert_eq!(sorted_devices.len(), edges.len());
    
    let mut paths: HashMap<Device, i64> = edges.keys().map(|d| (d.clone(), 0)).collect();
    *paths.get_mut(&YOU).unwrap() = 1;

    for d in sorted_devices {
        let current_p = *paths.get(&d).unwrap();
        for e in edges.get(&d).unwrap() {
            *paths.get_mut(e).unwrap() += current_p;
        }
    }

    *paths.get(&OUT).unwrap()
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

}
