use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let total: i32 = io::BufReader::new(file)
        // Get every line in the file
        .lines()
        // Convert the line to a string, and then the string to an int, panicking if an error occurs
        .map(|x| x.expect("Invalid file").parse::<i32>().expect("Invalid Input"))
        // Perfoming math
        .map(|x| x / 3 - 2)
        // Totalling the numbers
        .sum();
    
    println!("{}", total);
}
