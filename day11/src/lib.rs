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

            let tos = iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|to_str| {
                    let mut to: Device = [0; 3];
                    to.copy_from_slice(to_str.as_bytes());
                    to
                })
                .collect();
            (from, tos)
        })
        .collect()
}

fn part1(edges: HashMap<Device, Vec<Device>>) -> i64 {
    let sorted_devices = topo_sort(&edges);

    let mut paths: HashMap<Device, i64> = edges.keys().map(|d| (d.clone(), 0)).collect();
    *paths.get_mut(&YOU).unwrap() = 1;

    for d in sorted_devices {
        let current_p = *paths.get(&d).unwrap();
        if let Some(tos) = edges.get(&d) {
            for e in tos {
                paths
                    .entry(*e)
                    .and_modify(|c| *c += current_p)
                    .or_insert(current_p);
            }
        }
    }

    *paths.get(&OUT).unwrap()
}

fn topo_sort(edges: &HashMap<Device, Vec<Device>>) -> Vec<Device> {
    let mut sorted_devices: Vec<Device> = Vec::new();
    let mut in_degree: HashMap<Device, i64> = edges.keys().map(|d| (d.clone(), 0)).collect();
    for e in edges.values().flat_map(|v| v.iter()) {
        in_degree.entry(*e).and_modify(|d| *d += 1).or_insert(1);
    }

    let mut queue = VecDeque::new();
    for (device, degree) in in_degree.iter() {
        if *degree == 0 {
            queue.push_back(*device);
            sorted_devices.push(*device);
        }
    }
    while let Some(d) = queue.pop_front() {
        if let Some(tos) = edges.get(&d) {
            for e in tos {
                let i = in_degree.get_mut(e).unwrap();
                *i -= 1;
                if *i == 0 {
                    queue.push_back(*e);
                    sorted_devices.push(*e);
                }
            }
        }
    }
    sorted_devices
}

const SVR: Device = [b's', b'v', b'r'];
const FFT: Device = [b'f', b'f', b't'];
const DAC: Device = [b'd', b'a', b'c'];

fn part2(edges: HashMap<Device, Vec<Device>>) -> i64 {
    let sorted_devices: Vec<Device> = topo_sort(&edges);

    let mut paths: HashMap<(Device, bool, bool), i64> = HashMap::new();
    paths.insert((SVR, false, false), 1);

    for d in sorted_devices {
        for fft in [false, true] {
            for dac in [false, true] {
                if let Some(&current_p) = paths.get(&(d, fft, dac)) {
                    if let Some(tos) = edges.get(&d) {
                        for e in tos {
                            let next = if e.eq(&FFT) {
                                (*e, true, dac)
                            } else if e.eq(&DAC) {
                                (*e, fft, true)
                            } else {
                                (*e, fft, dac)
                            };
                            paths
                                .entry(next)
                                .and_modify(|c| *c += current_p)
                                .or_insert(current_p);
                        }
                    }
                }
            }
        }
    }

    *paths.get(&(OUT, true, true)).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    use crate::parse_input;

    #[test_case("input/sample_input.txt", 5)]
    #[test_case("input/input.txt", 423)]
    fn test_part1(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part1(input);

        assert_eq!(result, epxected);
    }

    #[test_case("input/sample_input_2.txt", 2)]
    #[test_case("input/input.txt", 333657640517376)]
    fn test_part2(input_file: &str, epxected: i64) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let input = parse_input(&input_str);
        let result = crate::part2(input);

        assert_eq!(result, epxected);
    }
}
