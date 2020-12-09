use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // Reading file into a vector of numbers, which I can then window over
    let numbers: Vec<i64> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    
    // Finding the first number in the set that isn't the sum of the previous 25 numbers
    const PREAMBLESIZE: usize = 25;
    for set in numbers.windows(PREAMBLESIZE + 1) {
        let mut seen = HashSet::with_capacity(PREAMBLESIZE);
        let mut found = false;
        for num in set.iter().take(PREAMBLESIZE) {
            seen.insert(num);
            if seen.contains(&(set[PREAMBLESIZE] - num)) {
                found = true;
                break;
            }
        }
        if !found {
            // Found an item without a match
            println!("{}", set[PREAMBLESIZE]);
            break;
        }
    }
}