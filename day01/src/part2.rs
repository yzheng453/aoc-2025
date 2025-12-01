use std::io;

fn main() {
    let lines = io::stdin().lines();
    let (_, count) = lines
        .filter_map(|l| l.ok())
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

    println!("{}", count);
}
