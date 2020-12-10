use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut adapters: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    // Adding the first adaptor
    adapters.push(0);
    // Sorting a list for later efficiency
    adapters.sort_unstable();
    // Then adding the final adapter
    adapters.push(adapters[adapters.len() - 1] + 3);
    // So for each item, I want to go up to 3 higher if possible, otherwise less
    let mut differences = vec![0, 0, 0];
    for increase in adapters.windows(2) {
        let diff = increase[1] - increase[0];
        differences[diff as usize - 1] += 1;
    }
    //println!("{:?}", adapters);
    println!("{:?}", differences);
    let result = differences[0] * differences[2];
    println!("{}", result);
}
