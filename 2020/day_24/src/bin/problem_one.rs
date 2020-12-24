use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;
use regex::Regex;

// I will define hex coordinates using the following system of 2 numbers:
// Up-Right/North-East, and Right/East. 
// This lets you treat the grid as essentially a rotated square grid, which is good enough for my purposes
type HexPos = [i64; 2];

fn main() {
    let dir_parse = Regex::new(r"e|w|se|sw|ne|nw").unwrap();
    // Create a set containing currently black tiles.
    // If a position is found in the input a second time, remove it to flip it back.
    let mut black_tiles: HashSet<HexPos> = HashSet::new();
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    for line in lines {
        let directions = line.unwrap();
        let mut cur: HexPos = [0; 2];
        for step in dir_parse.captures_iter(&directions) {
            match &step[0] {
                "ne" => cur[0] += 1,
                "e" => cur[1] += 1,
                "se" => {
                    cur[0] -= 1;
                    cur[1] += 1;
                }
                "sw" => cur[0] -= 1,
                "w" => cur[1] -= 1,
                "nw" => {
                    cur[0] += 1;
                    cur[1] -= 1;
                }
                x => panic!("Unexpected instruction {}", x),
            }
        }
        if !black_tiles.contains(&cur) {
            // Add to flip it black
            black_tiles.insert(cur);
        }
        else {
            black_tiles.remove(&cur);
        }
    }
    println!("{}", black_tiles.len());
}
