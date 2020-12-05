const BUF_SIZE: usize = 1024;

struct Instruction {
    arg_count: u8,
    out_to_last: bool,
    method: Box<dyn Fn(Vec<i32>) -> Option<i32>>,
}

pub fn string_to_intcode(instructions: &str) -> Option<[i32; BUF_SIZE]> {
    // Takes a string of comma separated ints, converts to a vector of ints
    let mut register: [i32; BUF_SIZE] = [0; BUF_SIZE];
    for (i, num) in instructions.split(",").enumerate() {
        register[i] = num.parse().ok()?;
    }
    Some(register)
}

pub fn execute_intcode(mut register: [i32; BUF_SIZE]) -> Option<i32> {
    // Executes intcode
    // Takes a vector of ints and executes
    let mut pos = 0;

    loop {
        let instr = match register[pos] % 100 {
            1 => Instruction { arg_count: 3, out_to_last: true, method: Box::new(instructions::add) },
            2 => Instruction { arg_count: 3, out_to_last: true, method: Box::new(instructions::mult) },
            3 => Instruction { arg_count: 1, out_to_last: true, method: Box::new(instructions::input) },
            4 => Instruction { arg_count: 1, out_to_last: false, method: Box::new(instructions::output) },
            99 => break,
            _ => return None,
        };
        // After parsing the instruction seeing if any other values matter
        let mut input = Vec::new();
        for arg in 0..(instr.arg_count as u32 - (if instr.out_to_last { 1 } else { 0 })) {
            if get_parameter(register[pos], arg) {
                // If get_parameter is true, get immediate value
                input.push(register[pos + arg as usize + 1]);
            }
            else {
                // Otherwise get position
                let pos = register[pos + arg as usize + 1] as usize;
                input.push(register[pos]);
            }
        }
        // After doing that, getting result
        let output = (instr.method)(input);
        if instr.out_to_last {
            if get_parameter(register[pos], instr.arg_count as u32 - 1) {
                register[pos + instr.arg_count as usize] = output.expect(&format!("Invalid output from {}", register[pos]));
            }
            else {
                let out_pos = register[pos + instr.arg_count as usize] as usize;
                register[out_pos] = output.expect(&format!("Invalid output from {}", register[pos]));
            }
        }
        pos += instr.arg_count as usize + 1;
    }
    
    Some(register[0])
}

fn get_parameter(instruction: i32, pos: u32) -> bool {
    (instruction / 10i32.pow(pos + 2)) % 10 == 1
}

mod instructions {
    use std::io;
    pub fn add(input: Vec<i32>) -> Option<i32> {
        Some(input.get(0)? + input.get(1)?)
    }

    pub fn mult(input: Vec<i32>) -> Option<i32> {
        Some(input.get(0)? * input.get(1)?)
    }

    pub fn input(_: Vec<i32>) -> Option<i32> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        Some(input.trim().parse::<i32>().ok()?)
    }

    pub fn output(input: Vec<i32>) -> Option<i32> {
        println!("{}", input[0]);
        None
    }
}