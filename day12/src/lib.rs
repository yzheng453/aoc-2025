use regex::Regex;

const PRESENT_SIZE: [i32; 6] = [6, 7, 5, 7, 7, 7];

fn solve(input_str: &String) -> i32 {
    let re = Regex::new(r"(\d+)x(\d+): (.*)").unwrap();
    input_str
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let (h, w): (i32, i32) = (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap());
            let v: Vec<i32> = caps.get(3).unwrap().as_str().split_whitespace().map(|n| n.parse().unwrap()).collect();
            assert!(v.len() == PRESENT_SIZE.len());
            let total_size = h * w;
            let minimum_size = v.iter().zip(PRESENT_SIZE.iter()).map(|(n, s)| *n * *s).sum();
            if total_size < minimum_size {
                return 0;
            }
            let num_3_by_3 = (h / 3) * (w / 3);
            if num_3_by_3 >= v.iter().sum() {
                return 1;
            }
            panic!("{}", line)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test_case::test_case;

    #[test_case("input/input.txt", 0)]
    fn test_part1(input_file: &str, epxected: i32) {
        let input_str = fs::read_to_string(input_file).unwrap();
        let result = crate::solve(&input_str);

        assert_eq!(result, epxected);
    }
}
