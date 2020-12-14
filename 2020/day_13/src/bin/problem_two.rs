use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // 
    let mut lines = io::BufReader::new(file)
        .lines();
    
    // Don't need the first line, ignoring it
    lines.next();
    
    // The list of busses, with the x's removed
    // (time, bus_no)
    let busses: Vec<(i64, i64)> = lines.next().unwrap().unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| {
            match x.parse() {
                Ok(num) => Some((num, i as i64)),
                Err(_) => None
            }
        })
        .collect();
    
    // I played around with maths and found a pattern that solves these problems very quickly
    let mut t = busses[0].0;
    let mut step = t as usize;
    for stop in busses.iter().skip(1) {
        for i in (t..).step_by(step) {
            if (i + stop.1) % stop.0 == 0 {
                // Found the match
                t = i;
                step *= stop.0 as usize;
                break;
            }
        }
    }
    println!("{}", t);
}
