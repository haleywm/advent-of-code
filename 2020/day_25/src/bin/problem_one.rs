use std::fs;
use std::io::{self, BufRead};
use day_25::*;

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    // So mucn unwrapping
    let card_key: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let door_key: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    
    // That's all our input, now to parse

    let card_loop = crack_loop(card_key);

    let result = forward_enc(door_key, card_loop);

    println!("{}", result);
}

