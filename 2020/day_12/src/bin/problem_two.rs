use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    // x, y
    let mut pos = (0, 0);
    let mut waypoint = (1, 10);

    for line in lines {
        let line = line.unwrap();
        // Get first char
        let instruction = line.chars().next().unwrap();
        // Turn rest of line into int
        let value: i32 = line[1..].parse().unwrap();
        match instruction {
            'N' => waypoint.0 += value,
            'S' => waypoint.0 -= value,
            'E' => waypoint.1 += value,
            'W' => waypoint.1 -= value,
            'L' => match value {
                90 => {
                    let new_x = waypoint.1;
                    let new_y = -waypoint.0;
                    waypoint.0 = new_x;
                    waypoint.1 = new_y;
                }
                180 => {
                    let new_x = -waypoint.0;
                    let new_y = -waypoint.1;
                    waypoint.0 = new_x;
                    waypoint.1 = new_y;
                }
                270 => {
                    let new_x = -waypoint.1;
                    let new_y = waypoint.0;
                    waypoint.0 = new_x;
                    waypoint.1 = new_y;
                }
                x => panic!("Unexpected value {}", x)
            }
            'R' => match value {
                90 => {
                    let new_x = -waypoint.1;
                    let new_y = waypoint.0;
                    waypoint.0 = new_x;
                    waypoint.1 = new_y;
                }
                180 => {
                    let new_x = -waypoint.0;
                    let new_y = -waypoint.1;
                    waypoint.0 = new_x;
                    waypoint.1 = new_y;
                }
                270 => {
                    let new_x = waypoint.1;
                    let new_y = -waypoint.0;
                    waypoint.0 = new_x;
                    waypoint.1 = new_y;
                }
                x => panic!("Unexpected value {}", x)
            }
            'F' => {
                pos.0 += waypoint.0 * value;
                pos.1 += waypoint.1 * value;
            }
            x => panic!("Unrecognized instruction {}", x),
        }
        //println!("{}, {}", pos.0, pos.1);

    }
    let result = pos.0.abs() + pos.1.abs();
    println!("{}", result);
}
