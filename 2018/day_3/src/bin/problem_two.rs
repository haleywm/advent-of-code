use std::fs;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

// Box has 5 values, the id, then the 4 corners
type Box = [usize; 5];

fn main() {
    // Line parser
    let line_parse = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    // Because I am lazy I'll just map coords to hash map
    let mut used: HashMap<(usize, usize), usize> = HashMap::new();
    let mut boxes: Vec<Box> = Vec::new();

    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    for line in lines {
        let line = line.unwrap();
        let vals = line_parse.captures(&line).unwrap();

        let id: usize = vals[1].parse().unwrap();
        let start_x: usize = vals[2].parse().unwrap();
        let start_y: usize = vals[3].parse().unwrap();
        let end_x = start_x + vals[4].parse::<usize>().unwrap();
        let end_y = start_y + vals[5].parse::<usize>().unwrap();

        for x in start_x..end_x {
            for y in start_y..end_y {
                let count = used.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }

        boxes.push([id, start_x, start_y, end_x, end_y]);
    }

    // Lastly, finding a box where all values are 1
    let result: usize = boxes.into_iter()
        .find_map(|guess| {
            for x in guess[1]..guess[3] {
                for y in guess[2]..guess[4] {
                    if *used.get(&(x, y)).unwrap() != 1 {
                        // No match
                        return None;
                    }
                }
            }
            // If we made it this far then this must be a match :D
            // Return the box ID
            Some(guess[0])
        })
        .unwrap();
    
    println!("{}", result);
}
