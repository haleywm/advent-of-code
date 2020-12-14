use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // 
    let mut lines = io::BufReader::new(file)
        .lines();
    
    // The earliest time I can arrive by
    let start: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // The list of busses, with the x's removed
    let busses: Vec<i32> = lines.next().unwrap().unwrap()
        .split(',')
        .filter_map(|x| {
            match x.parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        })
        .collect();
    
    // Bus time, bus id
    let mut earliest_bus = (0, 0);
    for time in busses {
        // Getting next possible time
        let next_time = (start as f64 / time as f64).ceil() as i32 * time;
        // If id is 0 get the next option so that there's an initial value
        if earliest_bus.1 == 0 || earliest_bus.0 > next_time {
            earliest_bus = (next_time, time);
        }
    }

    // Found the shortest bus time
    println!("{}", (earliest_bus.0 - start) * earliest_bus.1);
}
