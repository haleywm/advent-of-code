use regex::Regex;

enum Instruction {
    Hlf(bool),
    Tpl(bool),
    Inc(bool),
    Jmp(usize, bool),
    Jie(bool, usize, bool),
    Jio(bool, usize, bool),
}

fn main() {
    let re = Regex::new(r"^([a-z]+) (a|b|[+-]\d+)(?:, ([+-]\d+))?$").unwrap();
    let instructions: Vec<Instruction> = advent::line_iter("input/day_23.txt")
        .expect("Unable to open input file")
        .map(|line| {
            let line = line.unwrap();
            let cap = re.captures(&line).expect("Unable to parse line");
            match cap.get(1).unwrap().as_str() {
                "hlf" => {
                    Instruction::Hlf(
                        cap.get(2).unwrap().as_str() == "b"
                    )
                },
                "tpl" => {
                    Instruction::Tpl(
                        cap.get(2).unwrap().as_str() == "b"
                    )
                },
                "inc" => {
                    Instruction::Inc(
                        cap.get(2).unwrap().as_str() == "b"
                    )
                },
                "jmp" => {
                    let raw_num = cap.get(2).unwrap().as_str();
                    let pos = raw_num.as_bytes()[0] == b'+';
                    let num = raw_num[1..].parse().unwrap();
                    Instruction::Jmp(num, pos)
                },
                "jie" => {
                    let reg = cap.get(2).unwrap().as_str() == "b";
                    let raw_num = cap.get(3).unwrap().as_str();
                    let pos = raw_num.as_bytes()[0] == b'+';
                    let num = raw_num[1..].parse().unwrap();
                    Instruction::Jie(reg, num, pos)
                },
                "jio" => {
                    let reg = cap.get(2).unwrap().as_str() == "b";
                    let raw_num = cap.get(3).unwrap().as_str();
                    let pos = raw_num.as_bytes()[0] == b'+';
                    let num = raw_num[1..].parse().unwrap();
                    Instruction::Jio(reg, num, pos)
                },
                x => panic!("Unrecognized instruction: {}", x),
            }
        })
        .collect();
    
    println!("{}", run_program(&instructions, (0, 0)).1);
    println!("{}", run_program(&instructions, (1, 0)).1);
}

fn run_program(instructions: &[Instruction], register_values: (u64, u64)) -> (u64, u64) {
    let mut i: usize = 0;
    let mut registers: Vec<u64> = vec![register_values.0, register_values.1];
    loop {
        let mut inc = true;
        match instructions[i] {
            Instruction::Hlf(reg) => registers[reg as usize] /= 2,
            Instruction::Tpl(reg) => registers[reg as usize] *= 3,
            Instruction::Inc(reg) => registers[reg as usize] += 1,
            Instruction::Jmp(dif, dir) => {
                if dir {
                    i += dif;
                    inc = false;
                }
                else if dif > i {
                    // Would sub to negative
                    break;
                }
                else {
                    i -= dif;
                    inc = false;
                }
            },
            Instruction::Jie(reg, dif, dir) => {
                if registers[reg as usize] % 2 == 0 {
                    if dir {
                        i += dif;
                        inc = false;
                    }
                    else if dif > i {
                        // Would sub to negative
                        break;
                    }
                    else {
                        i -= dif;
                        inc = false;
                    }
                }
            },
            Instruction::Jio(reg, dif, dir) => {
                if registers[reg as usize] == 1 {
                    if dir {
                        i += dif;
                        inc = false;
                    }
                    else if dif > i {
                        // Would sub to negative
                        break;
                    }
                    else {
                        i -= dif;
                        inc = false;
                    }
                }
            },
        }
        if inc {
            i += 1;
        }
        if i >= instructions.len() {
            break;
        }
    }

    (registers[0], registers[1])
}
