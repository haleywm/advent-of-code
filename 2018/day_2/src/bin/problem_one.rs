use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut twos = 0;
    let mut threes = 0;
    let lines = io::BufReader::new(file)
        .lines();
    
    for line in lines {
        let line = line.unwrap();
        let mut freq: HashMap<char, usize> = HashMap::with_capacity(line.len());
        for letter in line.chars() {
            // Insert 0 if this character isn't in the set already, then inc by 1
            let count = freq.entry(letter).or_insert(0);
            *count += 1;
        }
        // Find twos and threes
        // Making my own loop would be more efficient but I'm lazy and there's not that many chars
        if freq.values().find(|x| **x == 2).is_some() {
            twos += 1;
        }
        if freq.values().find(|x| **x == 3).is_some() {
            threes += 1;
        }
    }

    let result = twos * threes;
    println!("{}", result);
}
