use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    // x, y
    let mut pos = (0, 0);
    // 0-360 direction
    let mut dir = 90;

    for line in lines {
        let line = line.unwrap();
        // Get first char
        let instruction = line.chars().next().unwrap();
        // Turn rest of line into int
        let value: i32 = line[1..].parse().unwrap();
        match instruction {
            'N' => pos.0 += value,
            'S' => pos.0 -= value,
            'E' => pos.1 += value,
            'W' => pos.1 -= value,
            'L' => dir -= value,
            'R' => dir += value,
            'F' => {
                match dir {
                    0 => pos.0 += value,
                    90 => pos.1 += value,
                    180 => pos.0 -= value,
                    270 => pos.1 -= value,
                    x => panic!("Invalid angle: {}", x),
                }
            }
            x => panic!("Unrecognized instruction {}", x),
        }
        // Correcting dir as needed
        dir = ((dir % 360) + 360) % 360;
        //println!("{}, {}", pos.0, pos.1);

    }
    let result = pos.0.abs() + pos.1.abs();
    println!("{}", result);
}
