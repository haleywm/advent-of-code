use day_3::{get_collision_count, line_to_bool_vec};
use std::fs;
use std::io::{self, BufRead};

const FILENAME: &str = "input.txt";
const SLOPE: (i32, i32) = (3, 1);

fn main() {
    let file = fs::File::open(FILENAME).expect("Invalid Filename");
    let lines = io::BufReader::new(file)
        .lines()
        .map(|x| line_to_bool_vec(&x.unwrap()));

    println!("{}", get_collision_count(lines, SLOPE));
}
