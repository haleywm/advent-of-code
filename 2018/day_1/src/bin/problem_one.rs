use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines();
    
    let result: i64 = lines
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .sum();
    
    println!("{}", result);
}
