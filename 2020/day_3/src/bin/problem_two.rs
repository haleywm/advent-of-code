use std::io::{self, BufRead};
use std::fs;
use day_3::{line_to_bool_vec, get_collision_count};

const FILENAME: &str = "input.txt";
const SLOPES: Vec<(i32, i32)> = vec![
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 1)
];

fn main() {
    let mut total = 1;
    for slope in SLOPES {
        let file = fs::File::open(FILENAME).expect("Invalid Filename");
        let lines = io::BufReader::new(file).lines()
            .map(|x| line_to_bool_vec(&x.unwrap()));
        total *= get_collision_count(lines, slope)
    }
    println!("{}", total);
}
