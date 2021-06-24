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
    pos: usize,
    input_needed: Option<usize>,
    memory: Vec<i64>
}

impl IntcodeMachine {
    pub fn new(data: Vec<i64>) -> IntcodeMachine {
        IntcodeMachine { pos: 0, input_needed: None, memory: data }
    }

    pub fn from_file(path: &str) -> Result<IntcodeMachine, &'static str> {
        let file = fs::read_to_string(path).or(Err("Failed to read file"))?;

        Ok(IntcodeMachine::try_from(file.as_str().trim())?)
    }

    pub fn execute_program(self: &mut Self) -> ExecReturn {
        loop {
            let instr = self.memory[self.pos];
            match instr % 100 {
                1 => {
                    let codes = split_opcodes(instr / 100, 3);
                    let addrs = self.translate_codes(codes);
                    self.memory[addrs[2]] = self.memory[addrs[0]] + self.memory[addrs[1]];
                    self.pos += 4;
                },
                2 => {
                    let codes = split_opcodes(instr / 100, 3);
                    let addrs = self.translate_codes(codes);
                    self.memory[addrs[2]] = self.memory[addrs[0]] * self.memory[addrs[1]];
                    self.pos += 4;
                },
                3 => {
                    let code = split_opcodes(instr / 100, 1);
                    let addr = self.translate_codes(code)[0];
                    self.input_needed = Some(addr);

                    self.pos += 2;

                    return ExecReturn::Input;
                },
                4 => {
                    let code = split_opcodes(instr / 100, 1);
                    let addr = self.translate_codes(code)[0];

                    self.pos += 2;
                    
                    return ExecReturn::Output(self.memory[addr]);
                },
                5 => {
                    let codes = split_opcodes(instr / 100, 2);
                    let addrs = self.translate_codes(codes);
                    if self.memory[addrs[0]] != 0 {
                        self.pos = self.memory[addrs[1]] as usize;
                    }
                    else {
                        self.pos += 3;
                    }
                },
                6 => {
                    let codes = split_opcodes(instr / 100, 2);
                    let addrs = self.translate_codes(codes);
                    if self.memory[addrs[0]] == 0 {
                        self.pos = self.memory[addrs[1]] as usize;
                    }
                    else {
                        self.pos += 3;
                    }
                },
                7 => {
                    let codes = split_opcodes(instr / 100, 3);
                    let addrs = self.translate_codes(codes);
                    self.memory[addrs[2]] = (self.memory[addrs[0]] < self.memory[addrs[1]]) as i64;
                    self.pos += 4;
                },
                8 => {
                    let codes = split_opcodes(instr / 100, 3);
                    let addrs = self.translate_codes(codes);
                    self.memory[addrs[2]] = (self.memory[addrs[0]] == self.memory[addrs[1]]) as i64;
                    self.pos += 4;
                },
                99 => return ExecReturn::Exit,
                x => panic!("Unrecognized instruction '{}' at pos {}", x, self.pos)
            }
        }
    }

    pub fn give_input(self: &mut Self, input: i64) {
        match self.input_needed {
            Some(pos) => {
                self.memory[pos] = input;
                self.input_needed = None;
            },
            None => panic!("Input given without request")
        }
    }

    pub fn borrow_memory(self: &mut Self) -> &mut Vec<i64> {
        return &mut self.memory;
    }

    fn translate_codes(self: &Self, codes: Vec<bool>) -> Vec<usize> {
        let mut values = Vec::with_capacity(codes.len());

        for i in 0..codes.len() {
            values.push(self.pos + 1 + i);
            
            if !codes[i] {
                values[i] = self.memory[values[i]] as usize;
            }
        }

        values
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

fn split_opcodes(number: i64, count: usize) -> Vec<bool> {
    let mut ops = Vec::with_capacity(count);
    for i in 0..count as u32 {
        let digit = (number / 10i64.pow(i)) % 10;
        ops.push(digit == 1);
    }
    
    ops
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExecReturn {
    Exit,
    Output(i64),
    Input
}
