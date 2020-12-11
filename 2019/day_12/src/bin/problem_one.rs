use std::fs;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let parser = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut moons_pos: Vec<[i32; 3]> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let result = parser.captures(&x).unwrap();
            [result[1].parse::<i32>().unwrap(), result[2].parse::<i32>().unwrap(), result[3].parse::<i32>().unwrap()]
        })
        .collect();
    let mut moons_vel = vec![[0; 3]; moons_pos.len()];
    
    const STEPS: i32 = 1000;

    for _ in 0..STEPS {
        for (i, velocity) in moons_vel.iter_mut().enumerate() {
            let pos = &moons_pos[i];
            // Increasing values as needed
            // Iterating through every other meteor's position, filtering out the current one
            for other_pos in moons_pos.iter()
                .enumerate()
                .filter_map(|(j, val)| {
                if j != i {
                    Some(val)
                }
                else {
                    None
                }
            }) {
                for index in 0..pos.len() {
                    if pos[index] < other_pos[index] {
                        velocity[index] += 1;
                    }
                    else if pos[index] > other_pos[index] {
                        velocity[index] -= 1;
                    }
                }
            }
        }
        // Iterating a second time once the velocities have updated
        for (pos, vel) in moons_pos.iter_mut().zip(moons_vel.iter()) {
            for i in 0..pos.len() {
                pos[i] += vel[i];
            }
        }
    }

    // After looping through as many times as needed, getting the total energy
    let result: i32 = moons_pos.iter()
        .zip(moons_vel.iter())
        .map(|(pos, vel)| {
            let mut pos_total = 0;
            let mut vel_total = 0;
            for num in pos {
                pos_total += num.abs();
            }
            for num in vel {
                vel_total += num.abs();
            }
            pos_total * vel_total
        })
        .sum();
    println!("{}", result);
}
