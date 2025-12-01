use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut position = 50;
    let mut count = 0;
    for line in lines {
        let line = line.unwrap();
        let direction = &line.chars().nth(0).unwrap();
        let clicks: i32 = (&line[1..]).parse::<i32>().unwrap();
        position += match direction {
            'L' => clicks,
            'R' => -clicks,
            _ => panic!()
        };
        position = position.rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    }
    
    println!("{}", count);
}
