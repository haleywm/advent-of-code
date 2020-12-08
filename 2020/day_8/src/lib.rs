pub fn parse_line(line: &str) -> (usize, i64) {
    // The arg is the first 3 characters
    // The number is the rest of the string after a space following the arg
    let arg = &line[..3];
    // Parsing arg into a usize that indicates a particular instruction to save memory
    let arg = match arg {
        "nop" => 0,
        "acc" => 1,
        "jmp" => 2,
        e => panic!("Unrecognized instruction: {}", e),
    };
    let num = line[4..].parse().unwrap();
    (arg, num)
}

pub fn execute_code(instructions: Vec<(usize, i64)>) -> Result<i64, i64> {
    // Executes instructions, alongside loop detection
    let mut pos = 0;
    let mut accumulator = 0;
    // Creating a separate vector to keep track of lines that have been run
    let mut has_run = vec![false; instructions.len()];

    loop {
        if has_run[pos] {
            // About to run code for the second time, loop detected
            return Err(accumulator);
        }
        else {
            has_run[pos] = true;
        }
        match instructions[pos].0 {
            0 => {
                // Do nothing
            }
            1 => {
                // Add value to accumulator
                accumulator += instructions[pos].1;
            }
            2 => {
                // Jump
                // Doing a lot of typecasting because pos should be usize, which can't be negative
                // Subbing 1 to compensate for pos being incremented later
                pos = (pos as i64 + instructions[pos].1 - 1) as usize;
            }
            x => panic!("Unrecognized instruction: {}", x),
        }
        pos += 1;
        if pos >= instructions.len() {
            return Ok(accumulator);
        }
    }
}