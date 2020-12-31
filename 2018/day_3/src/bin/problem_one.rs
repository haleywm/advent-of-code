use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // Line parser
    let line_parse = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    // Because I am lazy I'll just map coords to hash map
    let mut used: HashMap<(usize, usize), usize> = HashMap::new();
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    for line in lines {
        let line = line.unwrap();
        let vals = line_parse.captures(&line).unwrap();

        let start_x: usize = vals[1].parse().unwrap();
        let start_y: usize = vals[2].parse().unwrap();
        let end_x = start_x + vals[3].parse::<usize>().unwrap();
        let end_y = start_y + vals[4].parse::<usize>().unwrap();

        for x in start_x..end_x {
            for y in start_y..end_y {
                let count = used.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }

    // Lastly, totalling the number of overused areas (parts used 2 or more times)
    let result = used.values()
        .filter(|x| **x >= 2)
        .count();
    
    println!("{}", result);
}
