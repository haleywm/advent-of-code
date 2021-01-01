use std::fs;

fn main() {
    // Reading input to a string
    let input = fs::read_to_string("input.txt").unwrap();

    // Checking each possible outcome
    let result = ('a'..'z')
        .map(|remove| {
            // Creating a new string with the selected units removed
            let reduced_input: String = input
                .chars()
                .filter(|comp| comp.to_ascii_lowercase() != remove)
                .collect();
            
            // Then processing it until complete, and returning the length
            fully_cycle(reduced_input).len()
        })
        .min()
        .unwrap();

    // Once stabilized, print input length
    println!("{}", result);
}

fn fully_cycle(input: String) -> String {
    let mut cur = input;
    let mut cont = true;
    
    while cont {
        let output = cycle(cur);
        cur = output.0;
        cont = output.1;
    }

    cur
}

fn cycle(mut input: String) -> (String, bool) {
    // Returns a tuple. The first item is a string, that is the result of processing the previous string
    // The second item is a bool, it will be true if elements were removed (once false, no more changes can occur)
    // as the string has then reached a stable state

    assert!(input.len() >= 2);
    // Output buffer
    let mut output = String::with_capacity(input.len());
    // If the string has changed
    let mut changed = false;
    // The two currently looked at chars
    let mut cur = [input.pop().unwrap(), input.pop().unwrap()];
    // The next index in cur to swap out
    let mut cur_pos = 0;
    // If the buffer shouldn't be appended after looping
    let mut ignore_cur = false;

    while input.len() > 0 {
        // If both items in cur are case opposite versions, dump and continue, otherwise insert one val, replace with next, and carry on
        if (cur[0].is_ascii_uppercase() ^ cur[1].is_ascii_uppercase()) && cur[0].to_ascii_lowercase() == cur[1].to_ascii_lowercase() {
            // Matches! Disposing of both
            changed = true;
            // May not be room for all
            if input.len() >= 2 {
                cur[cur_pos] = input.pop().unwrap();
                cur[1 - cur_pos] = input.pop().unwrap();
            }
            else {
                // Sticking the 1 or 0 remaining characters into output
                while input.len() > 0 {
                    output.push(input.pop().unwrap());
                }
                ignore_cur = true;
            }
        }
        else {
            // Doesn't match, sending to output and cycling
            output.push(cur[cur_pos]);
            cur[cur_pos] = input.pop().unwrap();
            cur_pos = 1 - cur_pos;
        }
    }

    if !ignore_cur {
        output.push(cur[cur_pos]);
        output.push(cur[1 - cur_pos]);
    }

    (output, changed)
}