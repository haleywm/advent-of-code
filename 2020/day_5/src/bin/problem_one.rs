use std::fs;
use std::io::{self, BufRead};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let result = io::BufReader::new(file)
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
        .max().unwrap();
    println!("{}", result);
}
