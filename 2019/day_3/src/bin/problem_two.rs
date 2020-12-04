use std::fs;
use std::io::{self, BufRead};
use day_3::{str_to_coor, find_collisions};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|x| str_to_coor(&x.unwrap()));
    let (a, b) = (lines.next().unwrap(), lines.next().unwrap());

    let result = find_collisions(&a, &b)
        .iter()
        // Convert each one to manhattan distance
        .map(|x| (x.2, x.3))
        .min_by_key(|x| x.0 + x.1).unwrap();
    
    println!("{}, {}", result.0, result.1);
    println!("{}", result.0 + result.1);
}
