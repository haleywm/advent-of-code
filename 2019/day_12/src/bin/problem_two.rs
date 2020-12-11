use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use num::Integer;

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
    let moons_orig_pos = moons_pos.clone();
    let mut moons_vel = vec![[0; 3]; moons_pos.len()];
    // To 'speed things up', I will look for a loop on each axis, and then get the lcm of the three numbers
    let mut loops_required: [u64; 3] = [0; 3];

    for dim in 0..3 {
        loop {
            loops_required[dim] += 1;
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
                    if pos[dim] < other_pos[dim] {
                        velocity[dim] += 1;
                    }
                    else if pos[dim] > other_pos[dim] {
                        velocity[dim] -= 1;
                    }
                }
            }
            // Iterating a second time once the velocities have updated
            for (pos, vel) in moons_pos.iter_mut().zip(moons_vel.iter()) {
                pos[dim] += vel[dim];
            }
            // Lastly, seeing if we've reached a loop
            let mut loop_found = true;
            for ((pos, orig_pos), vel) in moons_pos.iter().zip(moons_orig_pos.iter()).zip(moons_vel.iter()) {
                if pos[dim] != orig_pos[dim] || vel[dim] != 0 {
                    loop_found = false;
                    break;
                }
            }
            if loop_found {
                break;
            }
        }
    }


    // After looping through as many times as needed, getting the total required amount of loops
    let result = loops_required[0]
        .lcm(&loops_required[1])
        .lcm(&loops_required[2]);
    println!("{}", result);
}
