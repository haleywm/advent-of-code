use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    let mut seen = HashSet::new();
    let mut cur: i64 = 0;
    
    let mut numbers = lines
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .into_iter()
        .cycle();
    
    let result = loop {
        let num = numbers.next().unwrap();
        cur += num;
        if seen.contains(&cur) {
            // Found!
            break cur;
        }
        else {
            // Insert
            seen.insert(cur);
        }
    };
    
    println!("{}", result);
}
