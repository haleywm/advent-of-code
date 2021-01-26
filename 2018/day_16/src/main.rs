use std::fs;
use std::io::{self, BufRead};

const REG_LEN: usize = 4;
type Register = [usize; REG_LEN];

fn main() {
    let file = fs::File::open("input.txt").expect("Invalid Filename");
    let mut lines = io::BufReader::new(file)
        .lines();
    
    // Mapping possibilities
    let mut possible: Vec<Vec<usize>> = vec![(0..16).collect(); 16];
    
    let mut total_above = 0;
    while let Some(Ok(next)) = lines.next() {
        if next.len() == 0 {
            // Reached end of first section, stopping here for now
            break;
        }
        let before = line_to_reg(next);
        let instr = line_to_instr(lines.next().unwrap().unwrap());
        let after = line_to_reg(lines.next().unwrap().unwrap());
        // Getting rid of next line as well as that should be empty
        lines.next();

        let mut possibilities = 0;
        for op_code in 0..16 {
            let test_after = exec_opcode(before.clone(), op_code, instr[1], instr[2], instr[3]);
            if after == test_after {
                possibilities += 1;
            }
            else {
                // Get index of number if it's still in list and remove
                if let Ok(index) = possible[instr[0]].binary_search(&op_code) {
                    possible[instr[0]].remove(index);
                }
            }
        }
        if possibilities >= 3 {
            total_above += 1;
        }
    }
    println!("{}", total_above);

    // Next, reducing the amount further
    loop {
        let mut complete = true;
        for i in 0..possible.len() {
            if possible[i].len() == 1 {
                // This must be the match, remove this number from every other one
                for j in 0..possible.len() {
                    if j != i {
                        if let Ok(index) = possible[j].binary_search(&possible[i][0]) {
                            possible[j].remove(index);
                        }
                    }
                }
            }
            else {
                complete = false;
            }
        }
        if complete {
            // Done!
            break;
        }
    }
    
    // Then, lastly, executing the test program using the mapping
    let mut mem = [0; 4];
    lines.next();
    while let Some(Ok(line)) = lines.next() {
        let inst = line_to_instr(line);
        mem = exec_opcode(mem, possible[inst[0]][0], inst[1], inst[2], inst[3]);
    }
    println!("{}", mem[0]);
}

fn exec_opcode(mut reg: Register, op_code: usize, a: usize, b: usize, c: usize) -> Register {
    // Assuming opcodes map 0-15 in order for now
    assert!(a < reg.len());
    assert!(b < reg.len());
    assert!(c < reg.len());
    match op_code {
        // addr
        0 => reg[c] = reg[a] + reg[b],
        // addi
        1 => reg[c] = reg[a] + b,
        // mulr
        2 => reg[c] = reg[a] * reg[b],
        // muli
        3 => reg[c] = reg[a] * b,
        //banr
        4 => reg[c] = reg[a] & reg[b],
        // bani
        5 => reg[c] = reg[a] & b,
        // borr
        6 => reg[c] = reg[a] | reg[b],
        // bori
        7 => reg[c] = reg[a] | b,
        // setr
        8 => reg[c] = reg[a],
        // seti
        9 => reg[c] = a,
        // gtir
        10 => reg[c] = (a > reg[b]) as usize,
        // gtri
        11 => reg[c] = (reg[a] > b) as usize,
        // gtrr
        12 => reg[c] = (reg[a] > reg[b]) as usize,
        // eqir
        13 => reg[c] = (a == reg[b]) as usize,
        // eqri
        14 => reg[c] = (reg[a] == b) as usize,
        // eqrr
        15 => reg[c] = (reg[a] == reg[b]) as usize,
        // else
        x => panic!("Op Code {} not covered", x),
    }

    reg
}

fn line_to_reg(input: String) -> Register {
    let mut result: Register = [0; REG_LEN];
    for (i, num) in input[9..input.len() - 1]
        .split(", ")
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
        .take(result.len()) {
            result[i] = num;
        }

    result
}

fn line_to_instr(input: String) -> Register {
    // Because instructions are also 4 long just reusing the register data type
    let mut result: Register = [0; REG_LEN];
    for (i, num) in input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
        .take(result.len()) {
            result[i] = num;
        }

    result
}