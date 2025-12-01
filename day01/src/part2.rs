use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut position = 50;
    let mut count = 0;
    for line in lines {
        let line = line.unwrap();
        let direction = &line.chars().nth(0).unwrap();
        let clicks: i32 = (&line[1..]).parse::<i32>().unwrap();
        let rotation = match direction {
            'L' => -clicks,
            'R' => clicks,
            _ => panic!()
        };
        let new_position = position + rotation;
        
        let n_crosses = (position.div_euclid(100) - new_position.div_euclid(100)).abs();
        count += n_crosses;
        
        if (position == 0) && (rotation < 0) {
            count -= 1;
        }

        position = new_position.rem_euclid(100);

        if (position == 0) && (rotation < 0) {
            count += 1;
        }
    }
    
    println!("{}", count);
}
