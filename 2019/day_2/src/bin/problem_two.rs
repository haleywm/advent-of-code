use std::fs;
use std::io::{self, BufRead};
use day_2::{string_to_intcode, execute_intcode};

fn main() -> std::io::Result<()> {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line)?;
    let line = line.trim();

    // Bruteforcing every possibility because I can't think of another good way
    const GOAL: i32 = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut code = string_to_intcode(line).expect("Invalid file");
            code[1] = noun;
            code[2] = verb;
            if execute_intcode(code).unwrap_or(-1) == GOAL {
                println!("Noun: {}, Verb: {} ({})", noun, verb, noun * 100 + verb);
                return Ok(());
            }
        }
    }
    println!("No match ");
    Ok(())
}
