use regex::Regex;
use std::cmp::{max, min};
use std::fs;

fn main() {
    let parser = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let input = fs::read_to_string("input/day_17.txt").expect("Unable to read input file");

    let cap = parser.captures(&input).expect("Invalid input");

    // Ignoring X for now
    let y_start: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
    let y_end: i64 = cap.get(4).unwrap().as_str().parse().unwrap();

    println!("{}", solve_part_one(y_start, y_end));

    let x_start: i64 = cap.get(1).unwrap().as_str().parse().unwrap();
    let x_end: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
    println!("{}", solve_part_two(y_start, y_end, x_start, x_end));
}

fn solve_part_one(y_start: i64, y_end: i64) -> i64 {
    let high = max(y_start, y_end);
    let low = min(y_start, y_end);

    // Bruteforcing it, checking every y velocity up to a limit
    let max_limit = 1000;
    (1..=max_limit)
        .scan(0, |state, num| {
            // Produce every maximum height for each velocity
            *state += num;
            Some(*state)
        })
        .filter(|x| {
            // Check if falling from x height will fit within range
            let mut vel = 1;
            let mut pos = *x;
            while pos > high {
                pos -= vel;
                vel += 1;
            }
            // Checking if current pos will fit
            pos <= high && pos >= low
        })
        .last()
        .expect("Unable to find valid velocity")
}

fn solve_part_two(y_start: i64, y_end: i64, x_start: i64, x_end: i64) -> i64 {
    // Find the number of velocities that will end up in the range
    (0..=1000)
        .map(|x| (-1000..=1000).map(move |y| (x, y)))
        .flatten()
        .filter(|(vel_x, vel_y)| {
            let mut vel_x = *vel_x;
            let mut vel_y = *vel_y;
            let low_x = min(x_start, x_end);
            let high_x = max(x_start, x_end);
            let low_y = max(y_start, y_end);
            let high_y = min(y_start, y_end);
            let mut x = 0;
            let mut y = 0;

            while x <= high_x && y >= high_y {
                x += vel_x;
                y += vel_y;

                vel_y -= 1;
                if vel_x > 0 {
                    vel_x -= 1;
                }

                if x >= low_x && x <= high_x && y <= low_y && y >= high_y {
                    return true;
                }
            }
            false
        })
        .count() as i64
}
