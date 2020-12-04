use std::fs;
use day_4::valid_passport_strict;

fn main() {
    let total = fs::read_to_string("input.txt")
        .expect("Couldn't find input")
        .split("\n\n")
        .filter(|x| valid_passport_strict(x))
        .count();
    
    println!("{}", total);
}
