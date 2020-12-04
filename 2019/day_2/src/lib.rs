pub fn string_to_intcode(instructions: &str) -> Option<Vec<i32>> {
    // Takes a string of comma separated ints, converts to a vector of ints
    let mut register: Vec<i32> = Vec::new();
    for num in instructions.split(",") {
        register.push(num.parse().ok()?)
    }
    Some(register)
}

pub fn execute_intcode(mut register: Vec<i32>) -> Option<i32> {
    // Takes a vector of ints and executes
    let mut pos = 0;

    loop {
        // Making the exit instructin part of the loop so know it exists 
        match *register.get(pos)? {
            1 => {
                let a = *register.get(*register.get(pos + 1)? as usize)?;
                let b = *register.get(*register.get(pos + 2)? as usize)?;
                let pos_c = *register.get(pos + 3)? as usize;
                let c = register.get_mut(pos_c)?;
                *c = a + b;
            }
            2 => {
                let a = *register.get(*register.get(pos + 1)? as usize)?;
                let b = *register.get(*register.get(pos + 2)? as usize)?;
                let pos_c = *register.get(pos + 3)? as usize;
                let c = register.get_mut(pos_c)?;
                *c = a * b;
            }
            99 => break,
            _ => return None,
        }
        pos += 4;
    }
    
    Some(register[0])
}