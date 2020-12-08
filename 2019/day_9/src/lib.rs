use std::io::{self, BufRead, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use OwnOrRef::*;

pub const BUF_SIZE: usize = 8192;
type Reg<'a> = (&'a mut [i64; BUF_SIZE], i64);

pub enum OwnOrRef<T> {
    Own(T),
    Ref(Arc<Mutex<T>>),
}

pub fn string_to_intcode(instructions: &str) -> Option<[i64; BUF_SIZE]> {
    // Takes a string of comma separated ints, converts to a vector of ints
    let mut register: [i64; BUF_SIZE] = [0; BUF_SIZE];
    for (i, num) in instructions.split(",").enumerate() {
        register[i] = num.parse().ok()?;
    }
    Some(register)
}

pub fn std_intcode(input: &str) -> thread::JoinHandle<()> {
    // Simply parses a string, and runs the output and output on the intcode through stdio
    let (stdin_send, stdin_rec) = mpsc::channel();
    let (stdout_send, stdout_rec) = mpsc::channel();
    let intcode = string_to_intcode(input.trim()).expect("Invalid file");

    // Spawning the execution in its own thread
    let handle_one = thread::spawn(move || {
        execute_intcode(
            // The code to execute:
            intcode,
            // Input:
            Own(stdin_rec),
            // Output:
            Own(stdout_send),
        )
    });
    // Now spawning threads to pipe input and output as needed
    thread::spawn(move || {
        // Lock stdin to just this thread
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        loop {
            // Usually this loop will be terminated when main ends.
            let mut input = String::new();
            handle.read_line(&mut input).expect("Unable to read stdin");
            let result = input.trim().parse();
            // If fails to parse ignore and move on
            if let Ok(num) = result {
                if let Err(_) = stdin_send.send(num) {
                    // Reciever has closed, exiting
                    break;
                }
            }
        }
    });

    let handle_two = thread::spawn(move || {
        // Lock stdout to just this thread
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        loop {
            let output = stdout_rec.recv();
            match output {
                // Once the other end closes end
                Err(_) => break,
                Ok(num) => {
                    let string = num.to_string() + "\n";
                    handle
                        .write_all(string.as_bytes())
                        .expect("Unable to write to stdout");
                }
            }
        }
        // Waiting for the main thread to toally finish
        handle_one.join().unwrap();
    });

    // Passing the handle to the stdout printer so that all output gets printed
    handle_two
}

pub fn run_intcode(
    register: [i64; BUF_SIZE],
    input: OwnOrRef<mpsc::Receiver<i64>>,
    output: OwnOrRef<mpsc::Sender<i64>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || execute_intcode(register, input, output))
}

fn execute_intcode(
    mut register: [i64; BUF_SIZE],
    input: OwnOrRef<mpsc::Receiver<i64>>,
    output: OwnOrRef<mpsc::Sender<i64>>,
) {
    // Executes intcode
    // Takes a vector of ints and executes
    let mut pos = 0;

    // Register contains the register, and the relative base
    let register: &mut Reg = &mut (&mut register, 0);

    loop {
        match register.0[pos] % 100 {
            1 => {
                // Add
                let a = read_arg(register, pos, 1);
                let b = read_arg(register, pos, 2);
                save_to_arg(register, pos, 3, a + b);
                pos += 4;
            }
            2 => {
                // Mult
                let a = read_arg(register, pos, 1);
                let b = read_arg(register, pos, 2);
                save_to_arg(register, pos, 3, a * b);
                pos += 4;
            }
            3 => {
                // Wait for input from async sender
                let input = match input {
                    Own(ref input) => input.recv().expect("Unable to read line"),
                    Ref(ref input) => input.lock().unwrap().recv().expect("Unable to read line"),
                };
                save_to_arg(register, pos, 1, input);
                pos += 2;
            }
            4 => {
                // Send the output, and then continue without blocking
                match output {
                    Own(ref output) => output.send(read_arg(register, pos, 1)),
                    Ref(ref output) => output.lock().unwrap().send(read_arg(register, pos, 1)),
                }
                .expect("Couldn't write output");
                //.expect("Unable to write out");
                pos += 2;
            }
            5 => {
                // Jump if True
                let test = read_arg(register, pos, 1);
                if test != 0 {
                    pos = read_arg(register, pos, 2) as usize;
                } else {
                    pos += 3;
                }
            }
            6 => {
                // Jump if False
                let test = read_arg(register, pos, 1);
                if test == 0 {
                    pos = read_arg(register, pos, 2) as usize;
                } else {
                    pos += 3;
                }
            }
            7 => {
                // Less Than
                let a = read_arg(register, pos, 1);
                let b = read_arg(register, pos, 2);
                save_to_arg(register, pos, 3, (a < b) as i64);
                pos += 4;
            }
            8 => {
                // Equals
                let a = read_arg(register, pos, 1);
                let b = read_arg(register, pos, 2);
                save_to_arg(register, pos, 3, (a == b) as i64);
                pos += 4;
            }
            9 => {
                // Set relative base
                let a = read_arg(register, pos, 1);
                register.1 += a;
                //eprintln!("Change: {}", register.1);
                pos += 2;
            }
            99 => break,
            x => {
                panic!("Invalid instruction: {}", x);
            }
        };
    }
}

fn get_parameter(instruction: i64, pos: u32) -> i64 {
    (instruction / 10i64.pow(pos + 1)) % 10
}

fn read_arg(register: &Reg, instr_pos: usize, arg: usize) -> i64 {
    match get_parameter(register.0[instr_pos], arg as u32) {
        0 => {
            let pos = register.0[instr_pos + arg] as usize;
            register.0[pos]
        }
        1 => register.0[instr_pos + arg],
        2 => {
            let pos = (register.0[instr_pos + arg] + register.1) as usize;
            register.0[pos]
        }
        x => panic!("Unrecognized parameter mode {}", x),
    }
}

fn save_to_arg(register: &mut Reg, instr_pos: usize, arg: usize, to_save: i64) {
    //let mut register = register.0;
    match get_parameter(register.0[instr_pos], arg as u32) {
        0 => {
            // Saving to the value pointed to
            let pos = register.0[instr_pos + arg] as usize;
            register.0[pos] = to_save;
        }
        1 => register.0[instr_pos + arg] = to_save,
        2 => {
            let pos = (register.0[instr_pos + arg] + register.1) as usize;
            register.0[pos] = to_save;
        }
        x => panic!("Unrecognized parameter mode {}", x),
    }
}
