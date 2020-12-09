use std::fs;
use std::io::{self, BufRead};
use num::integer::gcd;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // Translating string into a 2d vector of stuff
    let met_map: Vec<Vec<bool>> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let mut bool_vec = Vec::with_capacity(x.len());
            for item in x.chars() {
                if item == '#' {
                    bool_vec.push(true);
                }
                else {
                    bool_vec.push(false);
                }
            }
            bool_vec
        })
        .collect();
    
    // Next, seeing how many meteors are visible from each position
    let mut result: Vec<(u64, (u64, u64))> = Vec::new();
    for x in 0..met_map.len() {
        'parent_count: for y in 0..met_map[0].len() {
            // x across, y down
            if !met_map[x][y] {
                // Only checking from positions with meteors
                continue 'parent_count;
            }
            let mut total = 0;
            for comp_x in 0..met_map.len() {
                'child_test: for comp_y in 0..met_map[0].len() {
                    let diff = (comp_x as i64 - x as i64, comp_y as i64 - y as i64);
                    if met_map[comp_x][comp_y] && (diff.0 != 0 || diff.1 != 0) {
                        let place_count = gcd(diff.0, diff.1);
                        for mult in 1..place_count {
                            // If place_count is greater than 1, check each subspot for meteors
                            if met_map[((diff.0 / place_count * mult) + x as i64) as usize][((diff.1 / place_count * mult) + y as i64) as usize] {
                                // This spot is covered by a meteor
                                continue 'child_test;
                            }
                        }
                        // Passed tests, this is a meteor
                        total += 1;
                    }
                }
            }
            result.push((total, (x as u64, y as u64)));
        }
    }
    let high = result.iter().max_by_key(|x| x.0).unwrap();
    println!("{}: ({}, {})", high.0, high.1.0, high.1.1);
}
