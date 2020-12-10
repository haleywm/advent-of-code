use std::fs;
use std::io::{self, BufRead};
use num::integer::gcd;
use std::cmp::Ordering::Equal;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // Translating string into a 2d vector of stuff
    let mut met_map: Vec<Vec<bool>> = io::BufReader::new(file)
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
    
    let points = get_visible_by_all_points(&met_map);
    let high = points.iter().max_by_key(|x| x.0).unwrap();
    let point = high.1;
    //let point = (3, 8);
    println!("{}: ({}, {})", high.0, high.1.0, high.1.1);
    // After getting the optimal position, getting meteor destruction positions
    let mut destroyed = 0;
    const TARGET: usize = 200;
    loop {
        let mut to_destroy = get_visible_by_point(&met_map, point);
        if destroyed + to_destroy.len() < TARGET {
            // Order doesn't matter since a full rotation won't do it
            destroyed += to_destroy.len();
            for meteor in to_destroy {
                met_map[meteor.0][meteor.1] = false;
            }
        }
        else {
            // Sorting the last ones to find the exact one
            to_destroy.sort_unstable_by(|x, y| (y.1 as f32 - point.1 as f32).atan2((y.0 as f32 - point.0 as f32)).partial_cmp(&(x.1 as f32 - point.1 as f32).atan2((x.0 as f32 - point.0 as f32))).unwrap_or(Equal));
            //to_destroy.sort_by(|x, y| x.1.cmp(&y.1));
            let pos = TARGET - destroyed - 1;
            let item = to_destroy[pos];
            //println!("{:?}", to_destroy);
            println!("{}, {}: {}", item.0, item.1, item.1 * 100 + item.0);
            break;
        }

    }
}

fn get_visible_by_all_points(map: &Vec<Vec<bool>>) -> Vec<(usize, (usize, usize))> {
    let mut result: Vec<(usize, (usize, usize))> = Vec::new();
    for y in 0..map.len() {
        'parent_count: for x in 0..map[0].len() {
            // x across, y down
            if !map[y][x] {
                // Only checking from positions with meteors
                continue 'parent_count;
            }
            let total = get_visible_by_point(map, (y, x)).len();
            
            result.push((total, (y, x)));
        }
    }
    result
}

fn get_visible_by_point(map: &Vec<Vec<bool>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut total = Vec::new();
    for comp_y in 0..map.len() {
        'child_test: for comp_x in 0..map[0].len() {
            let diff = (comp_y as i64 - point.0 as i64, comp_x as i64 - point.1 as i64);
            if map[comp_y][comp_x] && (diff.0 != 0 || diff.1 != 0) {
                let place_count = gcd(diff.0, diff.1);
                for mult in 1..place_count {
                    // If place_count is greater than 1, check each subspot for meteors
                    if map[((diff.0 / place_count * mult) + point.0 as i64) as usize][((diff.1 / place_count * mult) + point.1 as i64) as usize] {
                        // This spot is covered by a meteor
                        continue 'child_test;
                    }
                }
                // Passed tests, this is a meteor
                total.push((comp_y, comp_x));
            }
        }
    }
    total
}