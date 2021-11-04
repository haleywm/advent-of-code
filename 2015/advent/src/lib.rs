use std::fs;
use std::io::{self, BufRead, BufReader, Lines};

pub fn line_iter(path: &str) -> io::Result<Lines<BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file)
        .lines();

    return Ok(reader);
}
