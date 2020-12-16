use std::fs;
use std::collections::{hash_map, HashMap};

fn main() {
    const TARGET: usize = 30000000;
    let numbers: Vec<usize> = fs::read_to_string("input.txt").unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    
    let mut seen: HashMap<usize, usize> = HashMap::new();
    
    // Setting inital values
    for (pos, num) in numbers.iter().enumerate().take(numbers.len() - 1) {
        seen.insert(*num, pos + 1);
    }
    let mut last = *numbers.last().unwrap();
    for pos in (numbers.len() + 1)..=TARGET {
        // Getting the next number based on the last number and seen
        match seen.entry(last) {
            hash_map::Entry::Occupied(mut entry) => {
                // Been seen before
                // Inserting a value returns the old one so doing both at once
                //println!("({} - {})", pos - 1, entry.get());
                last = (pos - 1) - entry.insert(pos - 1);
            }
            hash_map::Entry::Vacant(entry) => {
                // Hasn't been seen before
                last = 0;
                entry.insert(pos - 1);
            }
        }
        //println!("{}", last);
    }
    println!("{}: {}", TARGET, last);
}
