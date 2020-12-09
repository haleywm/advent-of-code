use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;

const PREAMBLESIZE: usize = 25;


fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    // Reading file into a vector of numbers, which I can then window over
    let numbers: Vec<i64> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    
    // Finding the first number in the set that isn't the sum of the previous 25 numbers
    let result = find_non_match(&numbers).expect("Unable to find non-match");

    let result = find_sequence(&numbers, result).expect("Couldn't find summing sequence");

    println!("{}-{}", result.0, result.1);
    let sub_numbers = &numbers[result.0..result.1];
    let min = sub_numbers.iter().min().unwrap();
    let max = sub_numbers.iter().max().unwrap();

    println!("{}", min + max);
}

fn find_non_match(numbers: &Vec<i64>) -> Option<i64> {
    for set in numbers.windows(PREAMBLESIZE + 1) {
        let mut seen = HashSet::with_capacity(PREAMBLESIZE);
        let mut found = false;
        for num in set.iter().take(PREAMBLESIZE) {
            seen.insert(num);
            if seen.contains(&(set[PREAMBLESIZE] - num)) {
                found = true;
                break;
            }
        }
        if !found {
            // Found an item without a match
            return Some(set[PREAMBLESIZE]);
        }
    }
    None
}

fn find_sequence(numbers: &Vec<i64>, total: i64) -> Option<(usize, usize)> {
    let mut start = 0;
    let mut end = 1;
    let mut sum = numbers[start] + numbers[end];
    while end < numbers.len() {
        if sum < total {
            // Need more
            end += 1;
            sum += numbers[end];
        }
        else if sum > total {
            // Need less
            sum -= numbers[start];
            start += 1;
        }
        else if start == end {
            // Came across a 1 length sequence oops, skipping to the next values and hopefully something will come up
            start += 1;
            end += 2;
            sum = numbers[start] + numbers[end];
        }
        else {
            // Found a match
            return Some((start, end));
        }
    }

    None
}
