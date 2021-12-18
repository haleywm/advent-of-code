use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let coord_parse = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let fold_parse = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();

    let file = fs::File::open("input/day_13.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file).lines().map(Result::unwrap);

    let mut points: HashSet<(i64, i64)> = lines
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let cap = coord_parse.captures(&line).expect("Invalid line");
            let x = cap.get(1).unwrap().as_str().parse().unwrap();
            let y = cap.get(2).unwrap().as_str().parse().unwrap();

            (x, y)
        })
        .collect();

    let mut first = true;
    for line in lines {
        let cap = fold_parse.captures(&line).unwrap();
        let axis = cap.get(1).unwrap().as_str();
        let fold_at: i64 = cap.get(2).unwrap().as_str().parse().unwrap();

        let new_points: HashSet<(i64, i64)> = if axis == "x" {
            points
                .iter()
                .map(|&(x, y)| {
                    let x = if x > fold_at {
                        fold_at - (fold_at - x).abs()
                    } else {
                        x
                    };
                    (x, y)
                })
                .collect()
        } else if axis == "y" {
            points
                .iter()
                .map(|&(x, y)| {
                    let y = if y > fold_at {
                        fold_at - (fold_at - y).abs()
                    } else {
                        y
                    };
                    (x, y)
                })
                .collect()
        } else {
            panic!("Idk");
        };

        points = new_points;
        // Part 1
        if first {
            first = false;
            println!("{}", points.len());
        }
    }

    // Now to print the final shape
    let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
