use std::fs;
use std::collections::VecDeque;

fn main() {
    const CUPS: usize = 1000000;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cups = VecDeque::with_capacity(CUPS);
    for label in input.chars() {
        cups.push_back(label.to_digit(10).unwrap() as i32);
    }
    for i in (cups.len() + 1)..=CUPS {
        cups.push_back(i as i32);
    }
    println!("Finished placing cups");

    const MOVES: usize = 10000;

    assert!(cups.len() >= 5);

    for _ in 0..MOVES {
        let cur = cups.pop_front().unwrap();
        let mut next_cups = Vec::with_capacity(3);
        for _ in 0..3 {
            next_cups.push(cups.pop_front().unwrap());
        }
        
        // Taking the current and next 3 elements off in the most efficient order, now to find the required insertion point
        // index, val
        let mut below = (0, -1);
        let mut max = (0, 0);
        for (i, val) in cups.iter().enumerate() {
            if *val > below.1 && *val < cur {
                // Closest below cur
                below = (i, *val);
                if below.1 == cur - 1 {
                    // Best match
                    break;
                }
            }
            if *val > max.1 {
                max = (i, *val);
            }
        }
        let dest = if below.1 != -1 {
            // Found a below value
            below.0
        }
        else {
            // Use max value
            max.0
        } + 1;
        for label in next_cups.into_iter().rev() {
            cups.insert(dest, label)
        }
        cups.push_back(cur);
    }
    // Lastly, printing the numbers as specified
    let idx = cups.iter().enumerate().find(|x| *x.1 == 1).unwrap().0 + 1;
    
    let val_one = cups[(idx + 1) % cups.len()] as u64;
    let val_two = cups[(idx + 2) % cups.len()] as u64;
    println!("{} * {} = {}", val_one, val_two, val_one * val_two);
}
