use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"(\d+)\D*(\d+)").unwrap();

    let input = fs::read_to_string("input/day_25.txt")
        .expect("Unable to read input file");
    
    let cap = re.captures(&input).expect("Failed to parse input");
    let row: u64 = cap.get(1).unwrap().as_str().parse().unwrap();
    let column: u64 = cap.get(2).unwrap().as_str().parse().unwrap();

    // First: Convert the row and column into a single index
    // Row starts at 1, and the number in the first column of 1 can be found by adding every number from 1 to row_num - 1
    // Column can then be found by summing every number from row+1 to row+column-1, then adding whatever the row started with
    let row_part = 1 + row * (row - 1) / 2;
    let to_perform = (row + column - 1) * (row + column) / 2 - ((row + 1) * row / 2) + row_part;

    let mut cur: u64 = 20151125;
    for _ in 0..(to_perform - 1) {
        cur = (cur * 252533) % 33554393;
    }

    println!("{}", cur);
}
