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
            let pos = match &step[0] {
                "ne" => 0,
                "e" => 1,
                "se" => 2,
                "sw" => 3,
                "w" => 4,
                "nw" => 5,
                x => panic!("Unexpected instruction {}", x),
            };
            change_pos(&mut cur, pos);
        }
        if !black_tiles.contains(&cur) {
            // Add to flip it black
            black_tiles.insert(cur);
        }
        else {
            black_tiles.remove(&cur);
        }
    }

    // After initializing state, now must simulate it
    const DAYS: usize = 100;
    for _ in 0..DAYS {
        // I'm going to use a hashmap for this because I can't be bothered recoding this in python
        // And I'm not touching rust's array tools
        let mut white_tiles = HashSet::with_capacity(black_tiles.len() * 6);
        for tile in black_tiles.iter() {
            // Getting the 6 adjacent tiles
            for i in 0..6 {
                let mut white = tile.clone();
                change_pos(&mut white, i);
                if !black_tiles.contains(&white) {
                    // Only adding adjacent tiles that aren't currently black
                    white_tiles.insert(white);
                }
            }
        }
        // Now that we have the relevant black and white tiles, seeing what meets the criteria
        let mut new_black_tiles: HashSet<HexPos> = HashSet::with_capacity(black_tiles.len() + white_tiles.len());

        for black in black_tiles.iter() {
            let adj = count_adj(black, &black_tiles);
            if adj == 1 || adj == 2 {
                // Stay black
                new_black_tiles.insert(*black);
            }
        }

        for white in white_tiles.into_iter() {
            let adj = count_adj(&white, &black_tiles);
            if adj == 2 {
                // Turn black
                new_black_tiles.insert(white);
            }
        }
        // Replacing old tiles with the new set
        black_tiles = new_black_tiles;
        //println!("Day {}: {}", run + 1, black_tiles.len());
    }
    println!("{}", black_tiles.len());
}

fn change_pos(pos: &mut HexPos, dif: usize) {
    match dif {
        0 => pos[0] += 1,
        1 => pos[1] += 1,
        2 => {
            pos[0] -= 1;
            pos[1] += 1;
        }
        3 => pos[0] -= 1,
        4 => pos[1] -= 1,
        5 => {
            pos[0] += 1;
            pos[1] -= 1;
        }
        _ => panic!("No"),
    }
}

fn count_adj(pos: &HexPos, set: &HashSet<HexPos>) -> usize {
    let mut count = 0;
    for i in 0..6 {
        let mut cur = pos.clone();
        change_pos(&mut cur, i);
        if set.contains(&cur) {
            count += 1;
        }
    }
    count
}