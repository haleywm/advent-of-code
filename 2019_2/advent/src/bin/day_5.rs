use advent::{IntcodeMachine, ExecReturn};

fn main() {
    let mut program_base = IntcodeMachine::from_file("input/day_5.txt").unwrap();

    assert_eq!(program_base.execute_program(), ExecReturn::Input);
    let mut program_two = program_base.clone();
    program_base.give_input(1);
    program_two.give_input(5);

    for (i, program) in [program_base, program_two].iter_mut().enumerate() {
        let mut prev = 0;
        loop {
            let result = program.execute_program();
            match result {
                ExecReturn::Output(num) => prev = num,
                ExecReturn::Exit => break,
                x => panic!("Unexpected return: {:?}", x)
            }
        }
        println!("Part {}: {}", i + 1, prev);
    }
}