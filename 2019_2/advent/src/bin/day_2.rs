use advent::IntcodeMachine;

const GOAL: i64 = 19690720;

fn main() {
    let mut program = IntcodeMachine::from_file("input/day_2.txt").unwrap();
    let program_save = program.clone();
    let mem = program.borrow_memory();
    mem[1] = 12;
    mem[2] = 2;

    program.execute_program();
    
    let mem = program.borrow_memory();
    println!("Part 1: {}", mem[0]);

    // Part 2
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut test = program_save.clone();
            let mem = test.borrow_memory();
            mem[1] = noun;
            mem[2] = verb;
            test.execute_program();
            let mem = test.borrow_memory();
            if mem[0] == GOAL {
                // Found it!
                println!("Part 2: {}", 100 * noun + verb);
                return;
            }
        }
    }
    // Failed to find it
    println!("Failed to find answer for Part 2");
}