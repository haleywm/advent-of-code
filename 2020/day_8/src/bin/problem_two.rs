use std::fs;
use std::io::{self, BufRead};
use day_8::{parse_line, execute_code};

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let instructions: Vec<(usize, i64)> = io::BufReader::new(file)
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    
    for line in (0..instructions.len()).filter(|x| instructions[*x].0 != 1) {
        // Bruteforcing every possible option
        // Filtering out the ones where the line is acc since those are correct
        let mut to_parse = instructions.clone();
        if to_parse[line].0 == 0 {
            to_parse[line].0 = 2;
        }
        else {
            to_parse[line].0 = 0;
        }
        if let Ok(res) = execute_code(to_parse) {
            // Found the solution
            println!("{}. Found solution by changing line {}", res, line);
            break;
        }

    }
}
