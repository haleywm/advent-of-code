use std::fs;
use std::io::{self, BufRead};
use day_8::{parse_line, execute_code};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let instructions: Vec<(usize, i64)> = io::BufReader::new(file)
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();

    let result = execute_code(instructions.clone());
    match result {
        Ok(val) => println!("{}: Finished successfully", val),
        Err(val) => println!("{}: Looped", val),
    }
}
