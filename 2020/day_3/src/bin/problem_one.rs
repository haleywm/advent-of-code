use std::io::{self, BufRead};
use std::fs;
use day_3::{line_to_bool_vec, get_collision_count};

const FILENAME: &str = "input.txt";
const SLOPE: (i32, i32) = (3, 1);

fn main() {
    let file = fs::File::open(FILENAME).expect("Invalid Filename");
    let lines = io::BufReader::new(file).lines()
        .map(|x| line_to_bool_vec(&x.unwrap()));

    println!("{}", get_collision_count(lines, SLOPE));
}
