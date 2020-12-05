use std::io::{self, BufRead, Write};

pub const BUF_SIZE: usize = 1024;

pub fn string_to_intcode(instructions: &str) -> Option<[i32; BUF_SIZE]> {
    // Takes a string of comma separated ints, converts to a vector of ints
    let mut register: [i32; BUF_SIZE] = [0; BUF_SIZE];
    for (i, num) in instructions.split(",").enumerate() {
        register[i] = num.parse().ok()?;
    }
    Some(register)
}

pub fn run_intcode(input: &str) {
    execute_intcode(
        // The code to execute:
        string_to_intcode(input.trim()).expect("Invalid file"),
        // Input:
        &mut io::stdin().lock(),
        // Output:
        &mut io::stdout()
    );
}


pub fn execute_intcode(mut register: [i32; BUF_SIZE], input: &mut impl BufRead, output: &mut impl Write) {
    // Executes intcode
    // Takes a vector of ints and executes
    let mut pos = 0;

    loop {
        match register[pos] % 100 {
            1 => {
                // Add
                let a = read_arg(&register, pos, 1);
                let b = read_arg(&register, pos, 2);
                save_to_arg(&mut register, pos, 3, a + b);
                pos += 4;
            },
            2 => {
                // Mult
                let a = read_arg(&register, pos, 1);
                let b = read_arg(&register, pos, 2);
                save_to_arg(&mut register, pos, 3, a * b);
                pos += 4;
            },
            3 => {
                // Input
                let mut input_buf = String::new();
                input.read_line(&mut input_buf).expect("Unable to read line");
                save_to_arg(&mut register, pos, 1, input_buf.trim().parse().expect("Invalid input"));
                pos += 2;
            },
            4 => {
                // Output
                let output_buf = read_arg(&register, pos, 1).to_string() + "\n";
                output.write_all(output_buf.as_bytes()).expect("Unable to write out");
                pos += 2;
            },
            5 => {
                // Jump if True
                let test = read_arg(&register, pos, 1);
                if test != 0 {
                    pos = read_arg(&register, pos, 2) as usize;
                }
                else {
                    pos += 3;
                }
            },
            6 => {
                // Jump if False
                let test = read_arg(&register, pos, 1);
                if test == 0 {
                    pos = read_arg(&register, pos, 2) as usize;
                }
                else {
                    pos += 3;
                }
            },
            7 => {
                // Less Than
                let a = read_arg(&register, pos, 1);
                let b = read_arg(&register, pos, 2);
                save_to_arg(&mut register, pos, 3, (a < b) as i32);
                pos += 4;
            },
            8 => {
                // Equals
                let a = read_arg(&register, pos, 1);
                let b = read_arg(&register, pos, 2);
                save_to_arg(&mut register, pos, 3, (a == b) as i32);
                pos += 4;
            },
            99 => break,
            _ => {
                println!("Invalid instruction");
                return;
            },
        };
    }
}

fn get_parameter(instruction: i32, pos: u32) -> bool {
    (instruction / 10i32.pow(pos + 1)) % 10 == 0
}

fn read_arg(register: &[i32; BUF_SIZE], instr_pos: usize, arg: usize) -> i32 {
    if get_parameter(register[instr_pos], arg as u32) {
        let pos = register[instr_pos + arg] as usize;
        register[pos]
    }
    else {
        register[instr_pos + arg]
    }
}

fn save_to_arg(register: &mut [i32; BUF_SIZE], instr_pos: usize, arg: usize, to_save: i32) {
    if get_parameter(register[instr_pos], arg as u32) {
        // Saving to the value pointed to
        let pos = register[instr_pos + arg] as usize;
        register[pos] = to_save;
    }
    else {
        register[instr_pos + arg] = to_save;
    }
}