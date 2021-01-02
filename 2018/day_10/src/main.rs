use std::fs;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let line_parse = Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    let mut positions: Vec<(i64, i64)> = Vec::new();
    let mut velocities: Vec<(i64, i64)> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let vals = line_parse.captures(&line).unwrap();

        let pos_x = vals[1].parse().unwrap();
        let pos_y = vals[2].parse().unwrap();
        let vel_x = vals[3].parse().unwrap();
        let vel_y = vals[4].parse().unwrap();

        positions.push((pos_x, pos_y));
        velocities.push((vel_x, vel_y));
    }

    // Each position matches a particular velocity
    assert_eq!(positions.len(), velocities.len());

    let mut prev_area = 0;
    let mut t = 0;
    // First attempt at a defition of found message:
    // The time with the lowest total area. If area is larger than prev value of t, go back one and that should be it
    loop {
        // First checking if message
        let mut min_x = positions[0].0;
        let mut max_x = positions[0].0;
        let mut min_y = positions[0].1;
        let mut max_y = positions[0].1;
        for pos in positions.iter() {
            if pos.0 < min_x {
                min_x = pos.0;
            }
            if pos.0 > max_x {
                max_x = pos.0;
            }
            if pos.1 < min_y {
                min_y = pos.1;
            }
            if pos.1 > max_y {
                max_y = pos.1;
            }
        }
        let area = (max_x - min_x) + (max_y - min_y);
        if prev_area != 0 && area > prev_area {
            for (pos, vel) in positions.iter_mut().zip(velocities.iter()) {
                pos.0 -= vel.0;
                pos.1 -= vel.1;
            }
            println!("{}", t - 1);

            let min_x = positions.iter().map(|x| x.0).min().unwrap();
            let max_x = positions.iter().map(|x| x.0).max().unwrap();
            let min_y = positions.iter().map(|x| x.1).min().unwrap();
            let max_y = positions.iter().map(|x| x.1).max().unwrap();
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if positions.contains(&(x, y)) {
                        print!("█");
                    }
                    else {
                        print!(" ");
                    }
                }
                println!();
            }
            break;
        }
        else {
            prev_area = area;
        }

        // Increasing values by velocity
        for (pos, vel) in positions.iter_mut().zip(velocities.iter()) {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }
        t += 1;
    }
}
