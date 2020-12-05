use std::fs;
use std::io::{self, BufRead};
use day_5::{string_to_intcode, execute_intcode};

fn main() -> std::io::Result<()> {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line)?;

    println!("{}", execute_intcode(string_to_intcode(line.trim()).expect("Invalid file")).unwrap_or(0));

    Ok(())
}
