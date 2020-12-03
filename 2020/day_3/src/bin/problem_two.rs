use day_3::{get_collision_count, line_to_bool_vec};
use std::fs;
use std::io::{self, BufRead};

#[macro_use]
extern crate lazy_static;

const FILENAME: &str = "input.txt";
lazy_static! {
    static ref SLOPES: Vec<(i32, i32)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
}

fn main() {
    let mut total: i64 = 1;
    for slope in &*SLOPES {
        let file = fs::File::open(FILENAME).expect("Invalid Filename");
        let lines = io::BufReader::new(file)
            .lines()
            .map(|x| line_to_bool_vec(&x.unwrap()));

        let result = get_collision_count(lines, *slope);
        println!("{}", result);
        total *= result as i64;
    }
    println!("{}", total);
}
