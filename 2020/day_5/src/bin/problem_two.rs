use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut values: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            let mut rows = 0;
            let mut cols = 0;
            for (i, dir) in x
                .unwrap()
                .chars()
                .enumerate() {
                if i < 7 {
                    if dir == 'B' {
                        rows += 2i32.pow(6 - i as u32);
                    }
                }
                else {
                    if dir == 'R' {
                        cols += 2i32.pow(9 - i as u32);
                    }
                }
            }
            rows * 8 + cols
        })
        .collect();
    // Sorting values, then finding the first missing gap
    values.sort_unstable();
    let result = values
        .windows(2)
        .find(|x| x[0] != x[1] - 1)
        .unwrap_or(&[-2])[0] + 1;
        
    println!("{}", result);
}
