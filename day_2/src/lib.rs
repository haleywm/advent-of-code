use regex::Regex;
use std::fs;
use std::io::{self, BufRead};

#[macro_use]
extern crate lazy_static;

pub fn process_file(filename: &str, func: fn(&str) -> Option<bool>) -> i32 {
    // Open file, check number of times it matches check_line
    let file = fs::File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    lines
        .map(|line| -> i32 {
            if let Ok(line) = line {
                func(&line).unwrap_or(false) as i32
            } else {
                0
            }
        })
        .sum()
}

pub fn check_line_minmax_match(line: &str) -> Option<bool> {
    // Static regex, only compiled once
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    // Parsing regex
    let result = RE.captures(line)?;
    // Can unwrap because the regex must have this as a valid int
    let min: usize = result[1].parse().unwrap();
    let max: usize = result[2].parse().unwrap();

    let count = result[4].matches(&result[3]).count();

    Some(count >= min && count <= max)
}

pub fn check_line_xor_match(line: &str) -> Option<bool> {
    // Static regex, only compiled once
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    // Parsing regex
    let result = RE.captures(line)?;
    // Can unwrap because the regex must have this as a valid int
    let one: usize = result[1].parse::<usize>().unwrap() - 1;
    let two: usize = result[2].parse::<usize>().unwrap() - 1;
    let ch: char = result[3].chars().next().unwrap();
    let mut chars = result[4].chars();
    
    let (one, two) = (
        chars.nth(one).unwrap(),
        chars.nth(two - one - 1).unwrap()
    );

    Some((one == ch) ^ (two == ch))
}
