use std::fs;
use std::io::{self, BufRead, BufReader, Lines};
use std::convert::TryFrom;

pub fn line_iter(path: &str) -> io::Result<Lines<BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file)
        .lines();

    return Ok(reader);
}

#[derive(Clone)]
pub struct IntcodeMachine {
    memory: Vec<i64>
}

impl IntcodeMachine {
    pub fn new(data: Vec<i64>) -> IntcodeMachine {
        IntcodeMachine { memory: data }
    }

    pub fn from_file(path: &str) -> Result<IntcodeMachine, &'static str> {
        let file = fs::read_to_string(path).or(Err("Failed to read file"))?;

        Ok(IntcodeMachine::try_from(file.as_str())?)
    }

    pub fn execute_program(self: &mut Self) {
        let mut pos = 0;

        loop {
            match self.memory[pos] {
                1 => {
                    let a: usize = self.memory[pos + 1] as usize;
                    let b: usize = self.memory[pos + 2] as usize;
                    let c: usize = self.memory[pos + 3] as usize;
                    self.memory[c] = self.memory[a] + self.memory[b];
                    pos += 4;
                },
                2 => {
                    let a: usize = self.memory[pos + 1] as usize;
                    let b: usize = self.memory[pos + 2] as usize;
                    let c: usize = self.memory[pos + 3] as usize;
                    self.memory[c] = self.memory[a] * self.memory[b];
                    pos += 4;
                },
                99 => break,
                x => panic!("Unrecognized instruction '{}' at pos {}", x, pos)
            }
        }
    }

    pub fn borrow_memory(self: &mut Self) -> &mut Vec<i64> {
        return &mut self.memory;
    }
}

impl TryFrom<&str> for IntcodeMachine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut numbers: Vec<i64> = Vec::new();
        for section in value.split(',') {
            numbers.push(section.parse().or(Err("Invalid number"))?);
        }

        Ok(IntcodeMachine::new(numbers))
    }
}