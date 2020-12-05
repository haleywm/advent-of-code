use std::fs;
use std::io::{self, BufRead};
use day_5::{string_to_intcode, execute_intcode, BUF_SIZE};
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line)?;
    let intcode = string_to_intcode(line.trim()).unwrap();
    let result = (0..5).permutations(5)
        .map(|x| {
            let mut prev = 0;
            for mode in x {
                prev = run_input(prev, mode, intcode);
            }
            prev
        })
        .max().unwrap();
    
    println!("{}", result);
    Ok(())
}

fn run_input(last_val: i32, mode: i32, line: [i32; BUF_SIZE]) -> i32 {
    let input = format!("{}\n{}\n", mode, last_val);
    let mut cursor = io::Cursor::new(input);
    let mut output: Vec<u8> = Vec::new();
    execute_intcode(line.clone(), &mut cursor, &mut output);
    let output = String::from_utf8(output).unwrap();
    //println!("{}", output);
    output.trim().parse().unwrap()
}
